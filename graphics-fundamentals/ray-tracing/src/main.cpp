#include <pngwriter.h>
#include <iostream>

#include "camera.hpp"
#include "materials/lambertian.hpp"
#include "rnd.hpp"
#include "surfaces/sphere.hpp"
#include "surfaces/world.hpp"

// "a function declared with constexpr is implicitly an inline function"
constexpr Vec3 linear_interp(const Vec3& start, const Vec3& end, float t) {
  return (1.0 - t) * start + t * end;
}

constexpr int max_ray_bounces = 2;

Vec3 ray_color(const Surface& surface, const Ray& r, int bounces) {
  const Vec3 white = Vec3(1.0, 1.0, 1.0);
  const Vec3 blue = Vec3(0.5, 0.7, 1.0);

  auto hit_result = surface.hit(r, 0.0, std::numeric_limits<float>::max());
  if (hit_result) {
    auto& [hit, material] = *hit_result;
    if (bounces < max_ray_bounces) {
      auto scatter_result = material->scatter(r, hit);
      if (scatter_result) {
        return scatter_result->attenuation.eltwise_mul(
            ray_color(surface, scatter_result->ray, bounces + 1));
      }
    }
    return Vec3(0, 0, 0);
  }

  float y_unit = r.direction().unit_vector().y;  // -1.0 < y < 1.0
  float t = 0.5 * (y_unit + 1.0);  // 0.5 * (0.0 < y < 2.0) = 0.0 < t < 1.0
  return linear_interp(white, blue, t);
}

int main(int, char**) {
  int width = 200, height = 100, samples_per_pixel = 8;

  Camera camera(samples_per_pixel);
  Rnd rnd;

  std::vector<std::unique_ptr<Surface>> surfaces;
  auto material =
      std::make_shared<Lambertian>(Vec3(0.5, 0.5, 0.5), rnd.gen_in_unit_sphere);
  surfaces.push_back(
      std::make_unique<Sphere>(Vec3(0.0, 0.0, -1.0), 0.5, material));
  surfaces.push_back(
      std::make_unique<Sphere>(Vec3(0.0, -100.5, -1), 100.0, material));
  auto world = World(std::move(surfaces));

  std::function<Vec3(const Ray&)> ray_color_fn = std::bind(
      ray_color, std::cref((Surface&)world), std::placeholders::_1, 0);

  pngwriter png(width, height, 0, "test.png");
  for (int y = height - 1; y >= 0; --y)
    for (int x = 0; x < width; ++x) {
      Vec3 color = camera.avgsample_pixel_color(x, y, width, height, rnd.gen,
                                                ray_color_fn);
      png.plot(x, y, color.r, color.g, color.b);
    }

  png.close();
}
