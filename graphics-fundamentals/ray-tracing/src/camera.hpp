#pragma once
#include <functional>
#include "ray.hpp"

class Camera {
 public:
  Camera(int num_samples_per_pixel)
      : origin(Vec3(0, 0, 0)),
        horizontal(Vec3(4, 0, 0)),
        vertical(Vec3(0, 2, 0)),
        lower_left_corner(Vec3(-2, -1, -1)),
        num_samples_per_pixel(num_samples_per_pixel) {}

  Ray ray(float u, float v) const {
    Vec3 direction = lower_left_corner + u * horizontal + v * vertical - origin;
    return Ray(origin, direction);
  }

  Vec3 avgsample_pixel_color(float x, float y, int width, int height,
                             std::function<float()> rnd,
                             std::function<Vec3(const Ray&)> ray_color) const {
    Vec3 color(0, 0, 0);
    for (int i = 0; i < num_samples_per_pixel; ++i) {
      float u = float(x + rnd()) / width;
      float v = float(y + rnd()) / height;
      Ray r = this->ray(u, v);
      color += ray_color(r);
    }

    return gamma_encode_color(color / float(num_samples_per_pixel));
  }

  static Vec3 gamma_encode_color(Vec3 color) {
    // out = in^gamma, "in most computer display systems, images are encoded
    // with a gamma of about 0.45". Let gamma be 0.5 so we can use the sqrt
    // (in^1/2) function
    return Vec3(std::sqrt(color.r), std::sqrt(color.g), std::sqrt(color.b));
  }

 private:
  Vec3 origin;
  Vec3 horizontal;
  Vec3 vertical;
  Vec3 lower_left_corner;
  int num_samples_per_pixel;
};
