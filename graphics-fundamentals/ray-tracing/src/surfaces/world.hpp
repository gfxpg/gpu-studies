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
  virtual std::optional<SurfaceHit> hit(const Ray& r, float t_min,
                                        float t_max) const {
    return std::accumulate(
        surfaces.cbegin(), surfaces.cend(), std::optional<SurfaceHit>(),
        [r, t_min, t_max](std::optional<SurfaceHit> closest_hit,
                          const std::unique_ptr<Surface>& s) {
          float t_limit = closest_hit ? closest_hit->t : t_max;
          auto hit = s->hit(r, t_min, t_limit);
          return hit ? hit : closest_hit;
        });
  }

 private:
  std::vector<std::unique_ptr<Surface>> surfaces;
};
