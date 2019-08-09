#pragma once
#include <optional>
#include "../ray.hpp"
#include "../surfaces/surface.hpp"

struct ScatteredRay {
  Ray ray;
  Vec3 attenuation;
};

class Material {
 public:
  virtual std::optional<ScatteredRay> scatter(
      const Ray& r, const SurfaceHit& surface_hit,
      std::function<float()> rnd) const = 0;
};
