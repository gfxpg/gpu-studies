#pragma once
#include <memory>
#include <numeric>
#include <vector>
#include "surface.hpp"

class World : public Surface {
 public:
  World() {}
  World(std::vector<std::unique_ptr<Surface>> surfaces)
      : surfaces(std::move(surfaces)) {}

  virtual SurfaceHitResult hit(const Ray& r, float t_min, float t_max) const {
    return std::accumulate(
        surfaces.cbegin(), surfaces.cend(), SurfaceHitResult(),
        [r, t_min, t_max](SurfaceHitResult closest_hit,
                          const std::unique_ptr<Surface>& s) {
          float t_limit = closest_hit ? closest_hit->first.t : t_max;
          auto hit = s->hit(r, t_min, t_limit);
          return hit ? hit : closest_hit;
        });
  }

 private:
  std::vector<std::unique_ptr<Surface>> surfaces;
};
