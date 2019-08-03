#include "sphere.hpp"

// The explanation is based on Fundamentals of Computer Graphics
// by Peter Shirley, Steve Marschner.

std::optional<SurfaceHit> Sphere::hit(const Ray& r, float t_min,
                                      float t_max) const {
  // The equation of a sphere is (x-x_c)^2 + (y-y_c)^2 + (z-z_c)^2 = R^2, where
  // c = (x_c,y_c,z_c) is the center, R is the radius. In vector form, it
  // becomes dot(p-c,p-c) = R^2. Any point p that satisfies the equation is on
  // the sphere. Given a ray p(t) = O+tD, the expanded form is
  // dot(D,D)t^2 + 2dot(D,O-C)t + dot(O-C,O-C) - R^2 = 0.

  // For a quadratic equation, the discriminant d=b^2-4ac shows how many real
  // solution there are:
  // * D > 0: two solutions, a) the ray enters the sphere, b) leaves the sphere
  // * D = 0: one solution, the ray touches the sphere at one point
  // * D < 0: no real solutions, the ray and the sphere do not intersect

  Vec3 oc = r.origin() - this->center;
  float a = r.direction().dot(r.direction());
  float b = 2.0 * r.direction().dot(oc);
  float c = oc.dot(oc) - this->radius * this->radius;

  float discriminant = b * b - 4.0 * a * c;
  if (discriminant < 0) return {};
  float t = (-b + std::sqrt(discriminant)) / a;
  if (t <= t_min || t >= t_max) t = (-b - std::sqrt(discriminant)) / a;
  if (t <= t_min || t >= t_max) return {};

  Vec3 pt = r.point_at(t);
  Vec3 surface_normal = (pt - this->center) / this->radius;

  return {SurfaceHit{t, pt, surface_normal}};
}
