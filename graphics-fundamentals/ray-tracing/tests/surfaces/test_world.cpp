#include <catch2/catch.hpp>

#include "../../src/surfaces/sphere.hpp"
#include "../../src/surfaces/world.hpp"

TEST_CASE("world z-order", "[surfaces]") {
  std::vector<std::unique_ptr<Surface>> surfaces;
  surfaces.push_back(std::make_unique<Sphere>(Vec3(1.0, 1.0, -5.0), 1.0));
  surfaces.push_back(std::make_unique<Sphere>(Vec3(1.0, 1.0, -9.0), 1.0));
  surfaces.push_back(std::make_unique<Sphere>(Vec3(1.0, 1.0, -3.0), 1.0));
  surfaces.push_back(std::make_unique<Sphere>(Vec3(1.0, 1.0, -7.0), 1.0));
  auto world = World(std::move(surfaces));

  Vec3 ray_origin = Vec3(1.0, 1.0, 0.0);
  Vec3 ray_direction = Vec3(0.0, 0.0, -1.0);
  auto hit = world.hit(Ray(ray_origin, ray_direction), 0.0,
                       std::numeric_limits<float>::max());

  REQUIRE(hit);
  REQUIRE(hit->p.x == 1.0f);
  REQUIRE(hit->p.y == 1.0f);
  REQUIRE(hit->p.z == Approx(-2.0f));
}
