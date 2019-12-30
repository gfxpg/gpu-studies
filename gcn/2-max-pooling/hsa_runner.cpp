#include <cstring>
#include <iostream>

#include "hsa_runner.hpp"
#include "hsa_utils.hpp"

HsaRunner::~HsaRunner()
{
  hsa_shut_down();
}

bool HsaRunner::init()
{
  hsa_status_t status = hsa_init();
  if (status != HSA_STATUS_SUCCESS)
    return hsa_error("hsa_init failed", status);

  HsaAgentEnumeration agents;
  status = hsa_iterate_agents(find_gpu_device, &agents);
  if (status != HSA_STATUS_SUCCESS || !agents.cpu.handle || !agents.gpu.handle)
    return hsa_error("Unable to find suitable HSA agents", status);
  _gpu_agent = agents.gpu;
  _cpu_agent = agents.cpu;

  char agent_name[64];
  status = hsa_agent_get_info(_gpu_agent, HSA_AGENT_INFO_NAME, agent_name);
  if (status != HSA_STATUS_SUCCESS)
    return hsa_error("Failed to get HSA_AGENT_INFO_NAME", status);
  std::cout << "Using agent: " << agent_name << std::endl;

  status = hsa_agent_get_info(_gpu_agent, HSA_AGENT_INFO_QUEUE_MAX_SIZE,
                              &_queue_size);
  if (status != HSA_STATUS_SUCCESS)
    return hsa_error("Failed to get HSA_AGENT_INFO_QUEUE_MAX_SIZE", status);

  status = hsa_queue_create(_gpu_agent, _queue_size, HSA_QUEUE_TYPE_MULTI, NULL,
                            NULL, UINT32_MAX, UINT32_MAX, &_queue);
  if (status != HSA_STATUS_SUCCESS)
    return hsa_error("hsa_queue_create failed", status);

  status = hsa_signal_create(1, 0, NULL, &_signal);
  if (status != HSA_STATUS_SUCCESS)
    return hsa_error("hsa_signal_create failed", status);

  return _mem.setup_memory_regions(_gpu_agent);
}

bool HsaRunner::setup_executable(const std::string& code_object_path,
                                 const std::string& symbol_name)
{
  if (!load_code_object(
          code_object_path,
          [&](size_t s) { return _mem.allocate_system_memory(s); },
          &_code_object))
    return false;

  hsa_status_t status = hsa_executable_create(
      HSA_PROFILE_FULL, HSA_EXECUTABLE_STATE_UNFROZEN, NULL, &_executable);
  if (status != HSA_STATUS_SUCCESS)
    return hsa_error("hsa_executable_create failed", status);

  status = hsa_executable_load_code_object(_executable, _gpu_agent,
                                           _code_object, NULL);
  if (status != HSA_STATUS_SUCCESS)
    return hsa_error("hsa_executable_load_code_object failed", status);

  status = hsa_executable_freeze(_executable, NULL);
  if (status != HSA_STATUS_SUCCESS)
    return hsa_error("hsa_executable_freeze failed", status);

  hsa_executable_symbol_t _symbol;
  status = hsa_executable_get_symbol(_executable, NULL, symbol_name.c_str(),
                                     _gpu_agent, 0, &_symbol);
  if (status != HSA_STATUS_SUCCESS)
    return hsa_error("hsa_executable_get_symbol failed", status);

  status = hsa_executable_symbol_get_info(
      _symbol, HSA_EXECUTABLE_SYMBOL_INFO_KERNEL_OBJECT, &_code_object_handle);
  if (status != HSA_STATUS_SUCCESS)
    return hsa_error("hsa_executable_symbol_get_info failed", status);

  status = hsa_executable_symbol_get_info(
      _symbol, HSA_EXECUTABLE_SYMBOL_INFO_KERNEL_GROUP_SEGMENT_SIZE,
      &_group_static_size);
  if (status != HSA_STATUS_SUCCESS)
    return hsa_error("hsa_executable_symbol_get_info failed", status);

  return true;
}

bool HsaRunner::setup_dispatch_packet(const KernelParams& params)
{
  const uint32_t queue_mask = _queue->size - 1;
  _dispatch_packet_index = hsa_queue_add_write_index_relaxed(_queue, 1);
  _dispatch_packet =
      (hsa_kernel_dispatch_packet_t*)(hsa_kernel_dispatch_packet_t*)(_queue
                                                                         ->base_address) +
      (_dispatch_packet_index & queue_mask);
  memset((uint8_t*)_dispatch_packet + 4, 0, sizeof(*_dispatch_packet) - 4);
  _dispatch_packet->completion_signal = _signal;
  _dispatch_packet->workgroup_size_x = params.workgroup_size_x;
  _dispatch_packet->workgroup_size_y = params.workgroup_size_y;
  _dispatch_packet->workgroup_size_z = params.workgroup_size_z;
  _dispatch_packet->grid_size_x = params.grid_x;
  _dispatch_packet->grid_size_y = params.grid_y;
  _dispatch_packet->grid_size_z = params.grid_z;
  _dispatch_packet->group_segment_size = 0;
  _dispatch_packet->private_segment_size = 0;
  _dispatch_packet->kernel_object = _code_object_handle;
  _dispatch_packet->kernarg_address = params.kernarg_ptr;
  return true;
}

bool HsaRunner::dispatch_kernel()
{
  uint16_t header =
      (HSA_PACKET_TYPE_KERNEL_DISPATCH << HSA_PACKET_HEADER_TYPE) |
      (1 << HSA_PACKET_HEADER_BARRIER) |
      (HSA_FENCE_SCOPE_SYSTEM << HSA_PACKET_HEADER_ACQUIRE_FENCE_SCOPE) |
      (HSA_FENCE_SCOPE_SYSTEM << HSA_PACKET_HEADER_RELEASE_FENCE_SCOPE);
  uint16_t dim = 1;
  if (_dispatch_packet->grid_size_y > 1)
    dim = 2;
  if (_dispatch_packet->grid_size_z > 1)
    dim = 3;
  _dispatch_packet->group_segment_size = _group_static_size;
  uint16_t setup = dim << HSA_KERNEL_DISPATCH_PACKET_SETUP_DIMENSIONS;
  uint32_t header32 = header | (setup << 16);
  __atomic_store_n((uint32_t*)_dispatch_packet, header32, __ATOMIC_RELEASE);
  hsa_signal_store_relaxed(_queue->doorbell_signal, _dispatch_packet_index);

  return true;
}

bool HsaRunner::wait(uint64_t timeout_sec)
{
  clock_t beg = clock();
  hsa_signal_value_t result;
  do
  {
    result = hsa_signal_wait_acquire(_signal, HSA_SIGNAL_CONDITION_EQ, 0, ~0ULL,
                                     HSA_WAIT_STATE_ACTIVE);
    clock_t clocks = clock() - beg;
    if (clocks > (clock_t)timeout_sec * CLOCKS_PER_SEC)
    {
      std::cout << "Kernel execution timed out, elapsed time: " << (long)clocks
                << std::endl;
      return false;
    }
  } while (result != 0);
  return true;
}
