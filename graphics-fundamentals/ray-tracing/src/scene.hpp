#pragma once
#include <memory>
#include "materials/glass.hpp"
#include "materials/lambertian.hpp"
#include "materials/metal.hpp"
#include "rnd.hpp"
#include "surfaces/sphere.hpp"
#include "surfaces/world.hpp"

class Scene {
 public:
  Scene() : rng() {}

  std::unique_ptr<Surface> generate_ray_tracing_in_one_weekend_scene(
      float prob_matte, float prob_metal) {
    std::vector<std::unique_ptr<Surface>> objs(4 /* bg + large spheres */);

    auto gray_bg = std::make_shared<Lambertian>(Vec3(0.5, 0.5, 0.5));
    auto glass = std::make_shared<Glass>(1.5);
    auto brown = std::make_shared<Lambertian>(Vec3(0.4, 0.2, 0.1));
    auto metal_large = std::make_shared<Metal>(Vec3(0.7, 0.6, 0.5), 0.0);

    objs[0] =
        std::make_unique<Sphere>(Vec3(0.0, -1000.0, 0.0), 1000.0, gray_bg);
    objs[1] = std::make_unique<Sphere>(Vec3(0.0, 1.0, 0.0), 1.0, glass);
    objs[2] = std::make_unique<Sphere>(Vec3(-4.0, 1.0, 0.0), 1.0, brown);
    objs[3] = std::make_unique<Sphere>(Vec3(4.0, 1.0, 0.0), 1.0, metal_large);

    for (int a = 0; a < 20; ++a) {
      for (int b = 0; b < 20; ++b) {
        Vec3 center((a - 10) + 0.9 * rng.random(), 0.2,
                    (b - 10) + 0.9 * rng.random());
        float material_prob = rng.random();

        if ((center - Vec3(4.0, 0.2, 0.0)).length() > 0.9) {
          std::shared_ptr<Material> mat;
          if (material_prob < prob_matte)
            mat = std::make_shared<Lambertian>(
                Vec3(rng.random() * rng.random(), rng.random() * rng.random(),
                     rng.random() * rng.random()));
          else if (material_prob < prob_metal)
            mat = std::make_shared<Metal>(
                Vec3(rng.random(), rng.random(), rng.random()),
                0.5 * rng.random());
          else
            mat = glass;

          objs.push_back(std::make_unique<Sphere>(center, 0.2, mat));
        }
      }
    }

    return std::make_unique<World>(std::move(objs));
  }

 private:
  Rnd rng;
};
