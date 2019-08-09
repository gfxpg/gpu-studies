#pragma once
#include <functional>
#include "ray.hpp"
#include "rnd.hpp"

class Camera {
 public:
  constexpr Camera(Vec3 eye_pos, Vec3 look_pos, Vec3 up_direction,
                   float vertical_fov_rad, float aspect, float aperture) {
    float half_height = std::tan(vertical_fov_rad / 2);
    float half_width = half_height * aspect;
    plane_w = (eye_pos - look_pos).unit_vector();
    plane_u = up_direction.cross(plane_w);
    plane_v = plane_w.cross(plane_u);

    float focus_dist = (eye_pos - look_pos).length();

    origin = eye_pos;
    horizontal = 2 * half_width * focus_dist * plane_u;
    vertical = 2 * half_height * focus_dist * plane_v;
    lower_left_corner = eye_pos - half_width * focus_dist * plane_u -
                        half_height * focus_dist * plane_v -
                        focus_dist * plane_w;
    lens_radius = aperture / 2;
  }

  Ray ray(float s, float t, std::function<float()> rnd) const {
    Vec3 lens_position = lens_radius * Rnd::random_in_unit_disk(rnd);
    Vec3 lens_offset = plane_u * lens_position.x + plane_v * lens_position.y;

    Vec3 direction = lower_left_corner + s * horizontal + t * vertical;
    return Ray(origin + lens_offset, direction - origin - lens_offset);
  }

  Vec3 avgsample_pixel_color(float x, float y, int width, int height,
                             std::function<float()> rnd,
                             std::function<Vec3(const Ray&)> ray_color,
                             int num_samples_per_pixel) const {
    Vec3 color(0, 0, 0);
    for (int i = 0; i < num_samples_per_pixel; ++i) {
      float u = float(x + rnd()) / width;
      float v = float(y + rnd()) / height;
      Ray r = this->ray(u, v, rnd);
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
  Vec3 plane_u, plane_v, plane_w;
  Vec3 origin, horizontal, vertical, lower_left_corner;
  float lens_radius{0};
};
