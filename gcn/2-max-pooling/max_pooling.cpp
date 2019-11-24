#include <cassert>
#include <cstring>
#include <iostream>

#include "hsa_runner.hpp"

#define CODE_OBJECT "max_pooling.co"
#define CODE_SYMBOL "max_pooling"

struct Kernargs
{
  uint64_t in_ptr, out_ptr;
  uint32_t kh, kw, w, outw;
} __attribute__((packed));

int main(int argc, const char** argv)
{
  uint32_t H = 4, W = 4, KH = 2, KW = 2, OUTH = H - KH + 1, OUTW = W - KW + 1;

  HsaRunner runner;
  assert(runner.init());
  assert(runner.setup_executable(CODE_OBJECT, CODE_SYMBOL));
  std::cout << "Executable loaded" << std::endl;

  auto kernarg = runner.memory().allocate_buffer<void*>(sizeof(Kernargs), true);
  auto in_buffer = runner.memory().allocate_buffer<float>(H * W * sizeof(float), false);
  auto out_buffer = runner.memory().allocate_buffer<float>(OUTH * OUTW * sizeof(float), false);

  assert(kernarg != NULL);
  assert(in_buffer != NULL);
  assert(out_buffer != NULL);

  for (uint32_t i = 0; i < H * W; ++i)
    in_buffer->At(i) = (float)i;

  assert(in_buffer->copy_to_device());

  Kernargs kernarg_contents =
      {.in_ptr = (uint64_t)in_buffer->device_ptr(),
       .out_ptr = (uint64_t)out_buffer->device_ptr(),
       .kh = KH,
       .kw = KW,
       .w = W,
       .outw = OUTW};
  std::memcpy(kernarg->host_ptr(), &kernarg_contents, sizeof(Kernargs));

  assert(kernarg->copy_to_device());
  std::cout << "Kernargs: KH = " << KH << ", KW = " << KW << ", W = " << W << ", OUTW = " << OUTW << std::endl;

  KernelParams params;
  params.kernarg_ptr = kernarg->device_ptr();
  params.grid_x = OUTH;
  params.grid_y = OUTW;
  params.workgroup_size_x = OUTH;
  params.workgroup_size_y = OUTW;

  assert(runner.setup_dispatch_packet(params));

  std::cout << "Running the kernel..." << std::endl;
  assert(runner.dispatch_kernel());
  assert(runner.wait(120));

  assert(out_buffer->copy_from_device(runner.cpu_agent()));

  bool valid = true;
  for (uint32_t i = 0; i < OUTH * OUTW; ++i)
  {
    unsigned int row = i / OUTW;
    unsigned int col = i % OUTW;

    float max = in_buffer->At(row * W + col);

    for (unsigned int a = 0; a < KH; ++a)
    {
      for (unsigned int b = 0; b < KW; ++b)
      {
        float el = in_buffer->At((row + a) * W + (col + b));
        if (el > max)
          max = el;
      }
    }

    if (out_buffer->At(i) != max)
    {
      std::cout << "Validation error at " << i << ": got " << out_buffer->At(i) << " expected " << max << std::endl;
      valid = false;
    }
  }
  std::cout << "Execution completed" << std::endl;
  return valid ? 0 : 1;
}
