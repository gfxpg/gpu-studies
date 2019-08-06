#pragma once
#include <functional>
#include <random>
#include "vec3.hpp"

class Rnd {
 public:
  Rnd()
      : mt_rnd(std::random_device()()),
        gen(std::bind(std::uniform_real_distribution<float>(0.0, 1.0), mt_rnd)),
        gen_in_unit_sphere(std::bind(random_in_unit_sphere, gen)) {}

  std::function<float()> gen;
  std::function<Vec3()> gen_in_unit_sphere;

  // A "rejection method" from Ray Tracing in One Weekend
  // see also https://math.stackexchange.com/a/3184454
  static Vec3 random_in_unit_sphere(std::function<float()> gen) {
    Vec3 p;
    do {
      p = 2.0 * Vec3(gen(), gen(), gen()) - Vec3(1.0, 1.0, 1.0);
    } while (p.squared_length() >= 1.0);
    return p;
  }

 private:
  std::mt19937 mt_rnd;
};
