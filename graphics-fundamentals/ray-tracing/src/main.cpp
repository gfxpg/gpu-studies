#include <pngwriter.h>
#include <iostream>

#include "ray.hpp"
#include "vec3.hpp"

// "a function declared with constexpr is implicitly an inline function"
constexpr Vec3 linear_interp(const Vec3& start, const Vec3& end, float t) {
  return (1.0 - t) * start + t * end;
}

Vec3 ray_color(const Ray& r) {
  const Vec3 white = Vec3(1.0, 1.0, 1.0);
  const Vec3 blue = Vec3(0.5, 0.7, 1.0);

  float y_unit = r.direction().unit_vector().y;  // -1.0 < y < 1.0
  float t = 0.5 * (y_unit + 1.0);  // 0.5 * (0.0 < y < 2.0) = 0.0 < t < 1.0
  return linear_interp(white, blue, t);
}

int main(int, char**) {
  int width = 200, height = 100;
  Vec3 lower_left_corner(-2.0, -1.0, -1.0);
  Vec3 horizontal(-lower_left_corner.x * 2, 0.0, 0.0);
  Vec3 vertical(0.0, -lower_left_corner.y * 2, 0.0);
  Vec3 origin(0.0, 0.0, 0.0);

  pngwriter png(width, height, 0, "test.png");

  for (int y = height - 1; y >= 0; --y)
    for (int x = 0; x < width; ++x) {
      float u = float(x) / width;
      float v = float(y) / height;
      Ray r(origin, lower_left_corner + u * horizontal + v * vertical);
      Vec3 color = ray_color(r);
      png.plot(x, y, color.r, color.g, color.b);
    }

  png.close();
}
