#define cimg_display 0
#include "vendor/CImg.h"

#include <cassert>

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
  auto kernarg =
      runner.memory().allocate_buffer<float>(buffer_size, true /* kernarg */);
  auto output = runner.memory().allocate_buffer<float>(buffer_size, false);

  assert(kernarg != NULL);
  assert(output != NULL);

  for (int x = 0; x < img.width(); ++x)
    for (int y = 0; y < img.height(); ++y) {
      (*kernarg)[(x + y * img.width()) * 3 + 0] = img(x, y, 0, 0);
      (*kernarg)[(x + y * img.width()) * 3 + 1] = img(x, y, 0, 1);
      (*kernarg)[(x + y * img.width()) * 3 + 2] = img(x, y, 0, 2);
    }

  KernelParams params;
  params.kernarg_ptr = kernarg->agent_ptr();
  params.grid_x = img.width();
  params.grid_y = img.height();
  params.workgroup_size_x = 2;
  params.workgroup_size_y = 2;

  assert(runner.setup_dispatch_packet(params));
  assert(runner.dispatch_kernel());

  // if (!CopyFrom(_out)) {
  //   output << "Error: failed to copy from local" << std::endl;
  //   return false;

  // for (int x = 0; x < _img.width(); ++x)
  //   for (int y = 0; y < _img.height(); ++y) {
  //     _img(x, y, 0, 0) = _out->Data<float>((x + y * _img.width()) * 3 + 0);
  //     _img(x, y, 0, 1) = _out->Data<float>((x + y * _img.width()) * 3 + 1);
  //     _img(x, y, 0, 2) = _out->Data<float>((x + y * _img.width()) * 3 + 2);
  //   }

  // _img.save(OUTPUT_IMAGE);

  // return true;
}
