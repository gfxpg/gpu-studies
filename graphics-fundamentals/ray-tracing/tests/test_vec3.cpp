#include <catch2/catch.hpp>

#include "../src/vec3.hpp"

TEST_CASE("vector addition", "[vec3]") {
  // evaluated at compile time
  constexpr auto a = Vec3(3.0, 5.0, 7.0);
  constexpr auto b = Vec3(2.0, 4.0, 6.0);
  constexpr auto mul = a + b;

  REQUIRE(mul.x == Approx(5.0));
  REQUIRE(mul.y == Approx(9.0));
  REQUIRE(mul.z == Approx(13.0));
}
