#pragma once
#include <algorithm>
#include "material.hpp"

class Metal : public Material {
 public:
  Metal(Vec3 albedo, float fuzziness)
      : albedo(albedo), fuzziness(std::min<float>(1.0, fuzziness)) {}

  virtual std::optional<ScatteredRay> scatter(
      const Ray& r, const SurfaceHit& surface_hit,
      std::function<float()> rnd) const {
    Vec3 reflected =
        reflect_ray(r.direction().unit_vector(), surface_hit.normal);
    // By randomizing the reflected direction within a small sphere, we can
    // control how fuzzy the reflections appear.
    Ray scattered =
        Ray(surface_hit.p,
            reflected + fuzziness * Rnd::random_in_unit_sphere(rnd));

    if (scattered.direction().dot(surface_hit.normal) > 0)
      return {{scattered, albedo}};

    return std::nullopt;
  }

  // For metals, the ray is not randomly scattered, rather it is reflected. The
  // angle of reflection equals the angle of incidence (between the falling ray
  // and the normal).
  static Vec3 reflect_ray(const Vec3& ray, const Vec3& normal) {
    return ray - 2.0 * ray.dot(normal) * normal;
  }

 private:
  Vec3 albedo;
  float fuzziness;
  std::function<Vec3()> random_in_unit_sphere;
};
