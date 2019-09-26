#define cimg_display 0
#include "vendor/CImg.h"

#include <cassert>
#include <iostream>

#include "hsa_runner.hpp"

#define CODE_OBJECT "test.co"
#define CODE_SYMBOL "hello_world.kd"
#define INPUT_IMAGE "kabanchik.bmp"
#define OUTPUT_IMAGE "kabanchik_out.bmp"

int main(int argc, const char** argv) {
  auto img = cimg_library::CImg<float>(INPUT_IMAGE);
  auto buffer_size = img.width() * img.height() * 3 * sizeof(float);

  HsaRunner runner;
  assert(runner.init());
  assert(runner.setup_executable(CODE_OBJECT, CODE_SYMBOL));
  std::cout << "Executable loaded" << std::endl;

  auto kernarg =
      runner.memory().allocate_buffer<float>(buffer_size, true /* kernarg */);
  auto output = runner.memory().allocate_buffer<float>(buffer_size, false);

  assert(kernarg != NULL);
  assert(output != NULL);

  uint64_t output_device_ptr = (uint64_t)output->agent_ptr();
  *(uint64_t*)(kernarg->system_ptr()) = output_device_ptr;

  for (int x = 0; x < img.width(); ++x)
    for (int y = 0; y < img.height(); ++y) {
      (*kernarg)[sizeof(uint64_t) + (x + y * img.width()) * 3 + 0] = img(x, y, 0, 0);
      (*kernarg)[sizeof(uint64_t) + (x + y * img.width()) * 3 + 1] = img(x, y, 0, 1);
      (*kernarg)[sizeof(uint64_t) + (x + y * img.width()) * 3 + 2] = img(x, y, 0, 2);
    }

  assert(kernarg->copy_to_device());
  std::cout << "Kernarg copied to device" << std::endl;

  KernelParams params;
  params.kernarg_ptr = kernarg->agent_ptr();
  params.grid_x = img.width();
  params.grid_y = img.height();
  params.workgroup_size_x = 2;
  params.workgroup_size_y = 2;

  assert(runner.setup_dispatch_packet(params));

  std::cout << "Running the kernel" << std::endl;
  assert(runner.dispatch_kernel());
  assert(runner.wait(120));

  assert(output->copy_from_device(runner.cpu_agent()));

  for (int x = 0; x < img.width(); ++x)
    for (int y = 0; y < img.height(); ++y) {
      img(x, y, 0, 0) = (*output)[(x + y * img.width()) * 3 + 0];
      img(x, y, 0, 1) = (*output)[(x + y * img.width()) * 3 + 1];
      img(x, y, 0, 2) = (*output)[(x + y * img.width()) * 3 + 2];
    }

  img.save(OUTPUT_IMAGE);
}
