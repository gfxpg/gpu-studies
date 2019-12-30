#include "hsa_utils.hpp"
#include "hsa_memory.hpp"
#include <fstream>
#include <iostream>

bool hsa_error(const std::string& message, hsa_status_t status)
{
  const char* info = 0;
  if (status != HSA_STATUS_SUCCESS)
    hsa_status_string(status, &info);
  std::cout << message << ": " << (info ? info : "unknown error") << std::endl;
  return false;
}

bool load_code_object(const std::string& path,
                      std::function<void*(size_t)> alloc,
                      hsa_code_object_t* co)
{
  std::ifstream in(path.c_str(), std::ios::binary | std::ios::ate);
  if (!in)
  {
    std::cout << "Failed to load code object from " << path << std::endl;
    return false;
  }
  size_t size = std::string::size_type(in.tellg());
  char* ptr = (char*)alloc(size);
  if (!ptr)
  {
    std::cout << "Failed to allocate memory for code object" << std::endl;
    return false;
  }
  in.seekg(0, std::ios::beg);
  std::copy(std::istreambuf_iterator<char>(in),
            std::istreambuf_iterator<char>(), ptr);

  hsa_status_t status = hsa_code_object_deserialize(ptr, size, NULL, co);
  if (status != HSA_STATUS_SUCCESS)
  {
    std::cout << "Failed to deserialize code object from " << path << std::endl;
    return false;
  }
  return true;
}

hsa_status_t find_gpu_device(hsa_agent_t agent, void* data)
{
  if (data == NULL)
    return HSA_STATUS_ERROR_INVALID_ARGUMENT;
  hsa_device_type_t dev;
  hsa_status_t status = hsa_agent_get_info(agent, HSA_AGENT_INFO_DEVICE, &dev);
  if (status == HSA_STATUS_SUCCESS)
  {
    if (dev == HSA_DEVICE_TYPE_GPU)
      ((HsaAgentEnumeration*)data)->gpu = agent;
    else if (dev == HSA_DEVICE_TYPE_CPU)
      ((HsaAgentEnumeration*)data)->cpu = agent;
  }
  return HSA_STATUS_SUCCESS;
}
