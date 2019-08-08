#pragma once
#include <functional>
#include "ray.hpp"

class Camera {
 public:
  constexpr Camera(Vec3 eye_pos, Vec3 look_pos, Vec3 up_direction,
                   float vertical_fov_rad, float aspect) {
    float half_height = std::tan(vertical_fov_rad / 2);
    float half_width = half_height * aspect;
    Vec3 w = (eye_pos - look_pos).unit_vector();
    Vec3 u = up_direction.cross(w);
    Vec3 v = w.cross(u);

    origin = eye_pos;
    horizontal = 2 * half_width * u;
    vertical = 2 * half_height * v;
    lower_left_corner = eye_pos - half_width * u - half_height * v - w;
  }

  Ray ray(float u, float v) const {
    Vec3 direction = lower_left_corner + u * horizontal + v * vertical - origin;
    return Ray(origin, direction);
  }

  Vec3 avgsample_pixel_color(float x, float y, int width, int height,
                             std::function<float()> rnd,
                             std::function<Vec3(const Ray&)> ray_color,
                             int num_samples_per_pixel) const {
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
};
