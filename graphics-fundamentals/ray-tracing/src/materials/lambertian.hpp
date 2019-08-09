#pragma once
#include "material.hpp"

// Lambertian is a diffusely reflecting (matte) surface, one that appears
// equally bright from all directions.
class Lambertian : public Material {
 public:
  Lambertian(Vec3 albedo) : albedo(albedo) {}

  // Diffuse objects may reflect the light in a random direction or absorb it
  // (the more light is absorbed, the darker the surface is.)
  virtual std::optional<ScatteredRay> scatter(
      const Ray& r, const SurfaceHit& surface_hit,
      std::function<float()> rnd) const {
    Vec3 target_scatter_point =
        surface_hit.p + surface_hit.normal + Rnd::random_in_unit_sphere(rnd);
    Ray scattered = Ray(surface_hit.p, target_scatter_point - surface_hit.p);

    return {{scattered, albedo}};
  }

 private:
  Vec3 albedo;
};
