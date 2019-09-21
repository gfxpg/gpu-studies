#pragma once

#include <string>
#include "hsa_memory.hpp"

struct KernelParams {
  uint16_t workgroup_size_x = 1;
  uint16_t workgroup_size_y = 1;
  uint16_t workgroup_size_z = 1;
  uint32_t grid_x = 1;
  uint32_t grid_y = 1;
  uint32_t grid_z = 1;
  void* kernarg_ptr = NULL;
};

class HsaRunner {
 public:
  HsaRunner() : _queue_size(0), _queue(0) {
    _cpu_agent.handle = 0;
    _gpu_agent.handle = 0;
    _signal.handle = 0;
  }
  bool init();
  bool setup_executable(const std::string& code_object_path,
                        const std::string& symbol_name);
  bool setup_dispatch_packet(const KernelParams& params);
  bool dispatch_kernel();
  bool wait(uint64_t timeout_sec);
  HsaMemory& memory() { return _mem; }
  hsa_agent_t cpu_agent() const { return _cpu_agent; }

 private:
  HsaMemory _mem;
  hsa_agent_t _gpu_agent;
  hsa_agent_t _cpu_agent;
  uint32_t _queue_size;
  hsa_queue_t* _queue;
  hsa_signal_t _signal;
  hsa_kernel_dispatch_packet_t* _dispatch_packet;
  uint64_t _dispatch_packet_index;
  uint64_t _code_object_handle;
  hsa_code_object_t _code_object;
  hsa_executable_t _executable;
  uint32_t _group_static_size;
};
