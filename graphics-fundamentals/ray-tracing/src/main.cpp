#include <pngwriter.h>
#include <functional>
#include <iostream>
#include <random>

#include "camera.hpp"
#include "surfaces/sphere.hpp"
#include "surfaces/world.hpp"

// "a function declared with constexpr is implicitly an inline function"
constexpr Vec3 linear_interp(const Vec3& start, const Vec3& end, float t) {
  return (1.0 - t) * start + t * end;
}

Vec3 ray_color(const Surface& surface, const Ray& r) {
  const Vec3 white = Vec3(1.0, 1.0, 1.0);
  const Vec3 blue = Vec3(0.5, 0.7, 1.0);

  auto hit = surface.hit(r, 0.0, std::numeric_limits<float>::max());
  if (hit)
    return 0.5 * Vec3(hit->normal.x + 1, hit->normal.y + 1, hit->normal.z + 1);

  float y_unit = r.direction().unit_vector().y;  // -1.0 < y < 1.0
  float t = 0.5 * (y_unit + 1.0);  // 0.5 * (0.0 < y < 2.0) = 0.0 < t < 1.0
  return linear_interp(white, blue, t);
}

int main(int, char**) {
  int width = 200, height = 100;

  std::vector<std::unique_ptr<Surface>> surfaces;
  surfaces.push_back(std::make_unique<Sphere>(Vec3(0.0, 0.0, -1.0), 0.5));
  surfaces.push_back(std::make_unique<Sphere>(Vec3(0.0, -100.5, -1), 100.0));
  auto world = World(std::move(surfaces));

  Camera camera;

  pngwriter png(width, height, 0, "test.png");

  std::random_device rnd_device;
  auto rnd_fn = std::bind(std::uniform_real_distribution<float>(0.0, 1.0),
                          std::mt19937(rnd_device()));
  std::function<Vec3(const Ray&)> ray_color_fn =
      std::bind(ray_color, std::cref((Surface&)world), std::placeholders::_1);

  for (int y = height - 1; y >= 0; --y)
    for (int x = 0; x < width; ++x) {
      Vec3 color = camera.avgsample_pixel_color(x, y, width, height, rnd_fn,
                                                ray_color_fn);
      png.plot(x, y, color.r, color.g, color.b);
    }

  png.close();
}
