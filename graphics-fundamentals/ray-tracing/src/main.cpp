#include "camera.hpp"
#include "image.hpp"
#include "rnd.hpp"
#include "scene.hpp"

// "a function declared with constexpr is implicitly an inline function"
constexpr Vec3 linear_interp(const Vec3& start, const Vec3& end, float t) {
  return (1.0 - t) * start + t * end;
}

constexpr int max_ray_bounces = 10;

Vec3 ray_color(const Surface& surface, const Ray& r, int bounces,
               std::function<float()> rnd) {
  const Vec3 white = Vec3(1.0, 1.0, 1.0);
  const Vec3 blue = Vec3(0.5, 0.7, 1.0);

  auto hit_result = surface.hit(r, 0.001, std::numeric_limits<float>::max());
  if (hit_result) {
    auto [hit, material] = *hit_result;
    if (bounces < max_ray_bounces) {
      auto scatter_result = material->scatter(r, hit, rnd);
      if (scatter_result) {
        return scatter_result->attenuation.eltwise_mul(
            ray_color(surface, scatter_result->ray, bounces + 1, rnd));
      }
    }
    return Vec3(0, 0, 0);
  }

  float y_unit = r.direction().unit_vector().y;  // -1.0 < y < 1.0
  float t = 0.5 * (y_unit + 1.0);  // 0.5 * (0.0 < y < 2.0) = 0.0 < t < 1.0
  return linear_interp(white, blue, t);
}

constexpr float radians(float degrees) { return degrees / 180.0 * M_PI; }

int main(int, char**) {
  constexpr int width = 300, height = 200, samples_per_pixel = 10;
  constexpr Camera camera(
      /* camera */ Vec3(13.0, 2.0, 3.0), /* looking at */ Vec3(0, 0, 0.0),
      /* up */ Vec3(0, 1, 0), /* fov */ radians(25),
      float(width) / float(height), /* aperture */ 1.0 / 8.0);

  Scene scene;
  auto world =
      std::shared_ptr<Surface>(scene.generate_ray_tracing_in_one_weekend_scene(
          /* probability of matte */ 0.6,
          /* probability of matte + metal */ 0.92));

  Image image(width, height, "test.ppm");

#pragma omp parallel
  {
    // RNG needs to be thread-local
    Rnd rnd;
    std::function<float()> rnd_float = std::bind(&Rnd::random, std::ref(rnd));
    std::function<Vec3(const Ray&)> ray_fn = [world, rnd_float](const Ray& r) {
      return ray_color(*world, r, 0, rnd_float);
    };

#pragma omp for collapse(2)
    for (int y = 0; y < height; ++y)
      for (int x = 0; x < width; ++x) {
        Vec3 color = camera.avgsample_pixel_color(
            x, y, width, height, rnd_float, ray_fn, samples_per_pixel);
        image.set_pixel(x, y, color);
      }
  }

  image.write_binary_ppm();
}
