#pragma once
#include <algorithm>
#include "material.hpp"

class Glass : public Material {
 public:
  Glass(float refraction_idx) : refraction_idx(refraction_idx) {}

  virtual std::optional<ScatteredRay> scatter(
      const Ray& r, const SurfaceHit& surface_hit,
      std::function<float()> rnd) const {
    static constexpr Vec3 attenuation = Vec3(1.0, 1.0, 1.0);

    Vec3 reflected = reflect_ray(r.direction(), surface_hit.normal);

    Vec3 outward_normal;
    float refr_idx, cos;
    if (r.direction().dot(surface_hit.normal) > 0) {
      outward_normal = -surface_hit.normal;
      refr_idx = refraction_idx;
      cos = r.direction().dot(surface_hit.normal) / r.direction().length();
    } else {
      outward_normal = surface_hit.normal;
      refr_idx = 1.0 / refraction_idx;
      cos = -r.direction().dot(surface_hit.normal) / r.direction().length();
    }

    auto refracted = refract_ray(r.direction(), outward_normal, refr_idx);
    if (refracted) {
      // Reflectance is the ration of the reflected intensity to the intensity
      // of the incoming (incident) light. Rays that are not reflected are
      // refracted.
      float reflectance = schlick(cos);
      if (rnd() >= reflectance)
        return {{Ray(surface_hit.p, *refracted), attenuation}};
    }
    return {{Ray(surface_hit.p, reflected), attenuation}};
  }

  // See the Metal material
  static Vec3 reflect_ray(const Vec3& ray, const Vec3& normal) {
    return ray - 2.0 * ray.dot(normal) * normal;
  }

  // ??? This part is taken straight from Ray Tracing in One Weekend and I don't
  // understand the equation it models.
  static std::optional<Vec3> refract_ray(const Vec3& ray, const Vec3& normal,
                                         float refr_idx) {
    Vec3 uv = ray.unit_vector();
    float dt = uv.dot(normal);
    float discriminant = 1.0 - refr_idx * refr_idx * (1.0 - dt * dt);

    if (discriminant <= 0.0) return {};

    Vec3 refracted =
        refr_idx * (uv - normal * dt) - normal * std::sqrt(discriminant);
    return {refracted};
  }

  // Schlick's equation approximates reflectance based on the angle (theta)
  // between the incident light direction and the normal.
  float schlick(float cos_theta) const {
    // Assuming n1 is air (1.0)
    float r0 = (1.0 - refraction_idx) / (1.0 + refraction_idx);
    float r0_sq = r0 * r0;
    return r0_sq + (1.0 - r0_sq) * std::pow((1.0f - cos_theta), 5);
  }

 private:
  float refraction_idx;
};
