#pragma once
#include <memory>
#include <optional>
#include "../ray.hpp"

struct SurfaceHit {
  float t;
  // Position of the hitpoint along the ray
  Vec3 p;
  // A surface normal is a vector perpendicular to the surface, pointing out
  // from its center. It represents the orientation of the surface with respect
  // to the light (the surface becomes brighter if oriented towards a light
  // source)
  Vec3 normal;
};

class Material;  // circular reference

typedef std::optional<std::pair<SurfaceHit, std::shared_ptr<Material>>>
    SurfaceHitResult;

class Surface {
 public:
  virtual ~Surface() {}
  virtual SurfaceHitResult hit(const Ray& r, float t_min,
                               float t_max) const = 0;
};
