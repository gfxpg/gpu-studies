#pragma once
#include "surface.hpp"

class Sphere : public Surface {
 public:
  Sphere() {}
  Sphere(Vec3 center, float radius) : center(center), radius(radius) {}
  virtual std::optional<SurfaceHit> hit(const Ray& r, float t_min,
                                        float t_max) const;

 private:
  Vec3 center;
  float radius;
};
