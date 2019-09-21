#pragma once

#include <functional>
#include <string>
#include "hsa.h"
#include "hsa_ext_amd.h"

bool hsa_error(const std::string& message, hsa_status_t status);
hsa_status_t find_gpu_device(hsa_agent_t agent, void* data);
bool load_code_object(const std::string& path,
                      std::function<void*(size_t)> alloc,
                      hsa_code_object_t* co);
