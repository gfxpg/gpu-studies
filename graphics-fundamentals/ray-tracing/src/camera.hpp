#pragma once
#include "ray.hpp"

class Camera {
 public:
  Camera()
      : origin(Vec3(0, 0, 0)),
        horizontal(Vec3(4, 0, 0)),
        vertical(Vec3(0, 2, 0)),
        lower_left_corner(Vec3(-2, -1, -1)) {}

  Ray ray(float u, float v) const {
    Vec3 direction = lower_left_corner + u * horizontal + v * vertical - origin;
    return Ray(origin, direction);
  }

  Vec3 avgsample_pixel_color(float x, float y, int width, int height,
                             std::function<float()> rnd,
                             std::function<Vec3(const Ray&)> ray_color) const {
    const int num_samples = 100;

    Vec3 color(0, 0, 0);
    for (int i = 0; i < num_samples; ++i) {
      float u = float(x + rnd()) / width;
      float v = float(y + rnd()) / height;
      Ray r = this->ray(u, v);
      color += ray_color(r);
    }

    return color / float(num_samples);
  }

 private:
  Vec3 origin;
  Vec3 horizontal;
  Vec3 vertical;
  Vec3 lower_left_corner;
};
