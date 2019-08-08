#include <pngwriter.h>
#include <iostream>
#include <math.h>

#include "camera.hpp"
#include "materials/glass.hpp"
#include "materials/lambertian.hpp"
#include "materials/metal.hpp"
#include "rnd.hpp"
#include "surfaces/sphere.hpp"
#include "surfaces/world.hpp"

// "a function declared with constexpr is implicitly an inline function"
constexpr Vec3 linear_interp(const Vec3& start, const Vec3& end, float t) {
  return (1.0 - t) * start + t * end;
}

constexpr int max_ray_bounces = 10;

Vec3 ray_color(const Surface& surface, const Ray& r, int bounces,
               std::function<Vec3()> rnd) {
  const Vec3 white = Vec3(1.0, 1.0, 1.0);
  const Vec3 blue = Vec3(0.5, 0.7, 1.0);

  auto hit_result = surface.hit(r, 0.001, std::numeric_limits<float>::max());
  if (hit_result) {
    auto [hit, material] = *hit_result;
    if (bounces < max_ray_bounces) {
      auto scatter_result = material->scatter(r, hit);
      if (scatter_result) {
        return scatter_result->attenuation.eltwise_mul(
            ray_color(surface, scatter_result->ray, bounces + 1, rnd));
      }
    }
    return Vec3(0, 0, 0);
  }

  float y_unit = r.direction().unit_vector().y;  // -1.0 < y < 1.0
  float t = 0.5 * (y_unit + 1.0);  // 0.5 * (0.0 < y < 2.0) = 0.0 < t < 1.0
  return linear_interp(white, blue, t);
}

constexpr float radians(float degrees) {
  return degrees / 180.0 * M_PI;
}

int main(int, char**) {
  constexpr int width = 200, height = 100, samples_per_pixel = 10;
  constexpr Camera camera(
      /* camera */ Vec3(-1.2, 1.0, 0.5), /* looking at */ Vec3(0, 0, -1),
      /* up */ Vec3(0, 1, 0), /* fov */ radians(75), float(width) / float(height));

  Rnd rnd;
  std::function<float()> rnd_float = std::bind(&Rnd::random, std::ref(rnd));
  std::function<Vec3()> rnd_sphere =
      std::bind(&Rnd::random_in_unit_sphere, std::ref(rnd));

  std::vector<std::unique_ptr<Surface>> surfaces;
  auto matte_green =
      std::make_shared<Lambertian>(Vec3(0.8, 0.8, 0.0), rnd_sphere);
  auto matte = std::make_shared<Lambertian>(Vec3(0.5, 0.5, 0.5), rnd_sphere);
  auto metal = std::make_shared<Metal>(Vec3(0.5, 0.5, 0.5), /* fuzziness */ 0.4,
                                       rnd_sphere);
  auto glass = std::make_shared<Glass>(1.5, rnd_float);
  surfaces.push_back(
      std::make_unique<Sphere>(Vec3(0.0, -100.5, -1), 100.0, matte_green));
  surfaces.push_back(
      std::make_unique<Sphere>(Vec3(0.0, 0.0, -1.1), 0.5, matte));
  surfaces.push_back(
      std::make_unique<Sphere>(Vec3(1.0, 0.0, -1.1), 0.5, metal));
  surfaces.push_back(
      std::make_unique<Sphere>(Vec3(-1.0, 0.0, -1.1), 0.5, glass));
  auto world = World(std::move(surfaces));

  std::function<Vec3(const Ray&)> ray_color_fn =
      std::bind(ray_color, std::cref((Surface&)world), std::placeholders::_1, 0,
                rnd_sphere);

  pngwriter png(width, height, 0, "test.png");
  for (int y = height - 1; y >= 0; --y)
    for (int x = 0; x < width; ++x) {
      Vec3 color = camera.avgsample_pixel_color(
          x, y, width, height, rnd_float, ray_color_fn, samples_per_pixel);
      png.plot(x, y, color.r, color.g, color.b);
    }

  png.close();
}
