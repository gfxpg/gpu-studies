#pragma once
#include "vec3.hpp"

class Ray {
 public:
  Ray() {}
  Ray(const Vec3& a, const Vec3& b) : a(a), b(b) {}

  Vec3 origin() const { return a; }
  Vec3 direction() const { return b; }
  Vec3 point_at(float t) const { return a + t * b; }

 private:
  Vec3 a, b;
};
