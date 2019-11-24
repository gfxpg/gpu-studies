#pragma once

#include "hsa.h"
#include "hsa_ext_amd.h"
#include <functional>
#include <string>

bool hsa_error(const std::string& message, hsa_status_t status);

bool load_code_object(const std::string& path,
                      std::function<void*(size_t)> alloc,
                      hsa_code_object_t* co);

struct HsaAgentEnumeration
{
  hsa_agent_t cpu;
  hsa_agent_t gpu;
};

hsa_status_t find_gpu_device(hsa_agent_t agent, void* data);
