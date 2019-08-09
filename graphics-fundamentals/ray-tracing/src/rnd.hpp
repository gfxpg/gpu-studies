#pragma once
#include <functional>
#include <random>
#include "vec3.hpp"

class Rnd {
 public:
  Rnd() : mt_rnd(std::random_device()()), distr(0.0, 1.0) {}

  float random() { return distr(mt_rnd); }

  // A "rejection method" from Ray Tracing in One Weekend
  // see also https://math.stackexchange.com/a/3184454
  Vec3 random_in_unit_sphere() {
    Vec3 p;
    do {
      p = 2.0 * Vec3(random(), random(), random()) - Vec3(1.0, 1.0, 1.0);
    } while (p.squared_length() >= 1.0);
    return p;
  }

  static Vec3 random_in_unit_disk(std::function<float()> rnd) {
    Vec3 p;
    do {
      p = 2.0 * Vec3(rnd(), rnd(), 0) - Vec3(1.0, 1.0, 0.0);
    } while (p.dot(p) >= 1.0);
    return p;
  }

 private:
  std::mt19937 mt_rnd;
  std::uniform_real_distribution<float> distr;
};
