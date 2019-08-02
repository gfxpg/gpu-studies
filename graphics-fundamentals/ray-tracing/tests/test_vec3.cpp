#include <catch2/catch.hpp>

#include "../src/vec3.hpp"

TEST_CASE("vector addition", "[vec3]") {
  // evaluated at compile time
  constexpr auto add = Vec3(3.0, 5.0, 7.0) + Vec3(2.0, 4.0, 6.0);

  REQUIRE(add.x == Approx(5.0f));
  REQUIRE(add.r == Approx(5.0f));
  REQUIRE(add.y == Approx(9.0f));
  REQUIRE(add.g == Approx(9.0f));
  REQUIRE(add.z == Approx(13.0f));
  REQUIRE(add.b == Approx(13.0f));
}

TEST_CASE("vector subtraction", "[vec3]") {
  constexpr auto sub = Vec3(3.0, 5.0, 7.0) - Vec3(6.0, 4.0, 2.0);

  REQUIRE(sub.x == Approx(-3.0f));
  REQUIRE(sub.y == Approx(1.0f));
  REQUIRE(sub.z == Approx(5.0f));
}

TEST_CASE("scalar vector multiplication", "[vec3]") {
  constexpr auto mul = Vec3(0.15, 0.25, 0.35) * 10.0f;
  constexpr auto mul2 = 10.0f * Vec3(0.15, 0.25, 0.35);

  REQUIRE(mul.x == Approx(1.5f));
  REQUIRE(mul.y == Approx(2.5f));
  REQUIRE(mul.z == Approx(3.5f));

  REQUIRE(mul2.x == Approx(1.5f));
  REQUIRE(mul2.y == Approx(2.5f));
  REQUIRE(mul2.z == Approx(3.5f));
}

TEST_CASE("scalar vector division", "[vec3]") {
  constexpr auto div = Vec3(0.15, 0.25, 0.35) / 10.0f;

  REQUIRE(div.x == Approx(0.015f));
  REQUIRE(div.y == Approx(0.025f));
  REQUIRE(div.z == Approx(0.035f));
}

TEST_CASE("vector dot product", "[vec3]") {
  constexpr auto dot = Vec3(1.0, 2.0, 3.0).dot(Vec3(1.0, 5.0, 7.0));

  REQUIRE(dot == Approx(32.0f));
}

TEST_CASE("vector negation", "[vec3]") {
  constexpr auto negated = -Vec3(0.1, 0.2, 0.3);

  REQUIRE(negated.x == Approx(-0.1f));
  REQUIRE(negated.y == Approx(-0.2f));
  REQUIRE(negated.z == Approx(-0.3f));
}

TEST_CASE("vector length", "[vec3]") {
  constexpr auto vec = Vec3(1.0, 2.0, 3.0);
  constexpr float len = vec.length();
  constexpr float len_sq = vec.squared_length();

  REQUIRE(len == Approx(3.74165f));
  REQUIRE(len_sq == Approx(14.0f));
}

TEST_CASE("unit vector", "[vec3]") {
  constexpr auto unit = Vec3(14.0, 6.0, 5.0).unit_vector();

  REQUIRE(unit.x == Approx(0.8733f));
  REQUIRE(unit.y == Approx(0.37427f));
  REQUIRE(unit.z == Approx(0.31189f));
}
