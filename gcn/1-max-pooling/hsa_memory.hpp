#pragma once

#include "hsa.h"
#include "hsa_ext_amd.h"
#include "hsa_utils.hpp"

template <typename T>
class HsaBuffer
{
  public:
  HsaBuffer<T>(size_t size, T* system_ptr, T* agent_ptr = NULL)
      : _size(size), _host_ptr(system_ptr), _device_ptr(agent_ptr) {}

  const T& At(size_t i) const { return _host_ptr[i]; }
  T& At(size_t i) { return _host_ptr[i]; }

  void* host_ptr() { return _host_ptr; }
  void* device_ptr() const { return _device_ptr; }
  bool allocated_on_agent() const { return _device_ptr != NULL; }
  size_t size() const { return _size; }

  bool copy_from_device(hsa_agent_t cpu_agent)
  {
    hsa_status_t status = hsa_memory_assign_agent(_device_ptr, cpu_agent,
                                                  HSA_ACCESS_PERMISSION_RW);
    if (status != HSA_STATUS_SUCCESS)
      return hsa_error("hsa_memory_assign_agent failed", status);

    status = hsa_memory_copy(_host_ptr, _device_ptr, _size);
    if (status != HSA_STATUS_SUCCESS)
      return hsa_error("hsa_memory_copy failed", status);

    return true;
  }

  bool copy_to_device()
  {
    hsa_status_t status = hsa_memory_copy(_device_ptr, _host_ptr, _size);
    if (status != HSA_STATUS_SUCCESS)
      return hsa_error("hsa_memory_copy failed", status);

    return true;
  }

  private:
  size_t _size;
  T* _host_ptr;
  T* _device_ptr;
};

class HsaMemory
{
  public:
  HsaMemory()
  {
    _kernarg_region.handle = 0;
    _system_region.handle = 0;
    _local_region.handle = 0;
    _gpu_local_region.handle = 0;
  }

  void* allocate_system_memory(size_t size)
  {
    void* p = NULL;
    hsa_status_t status = hsa_memory_allocate(_system_region, size, (void**)&p);
    return status == HSA_STATUS_SUCCESS ? p : NULL;
  }

  bool setup_memory_regions(hsa_agent_t& agent)
  {
    hsa_status_t status =
        hsa_agent_iterate_regions(agent, setup_memory_region, this);
    if (status != HSA_STATUS_SUCCESS || !_system_region.handle ||
        !_kernarg_region.handle)
      return hsa_error("Failed to setup memory regions", status);
    return true;
  }

  void set_system_region(hsa_region_t r) { _system_region = r; }
  void set_kernarg_region(hsa_region_t r) { _kernarg_region = r; }
  void set_gpu_local_region(hsa_region_t r) { _gpu_local_region = r; }
  void set_local_region(hsa_region_t r) { _local_region = r; }

  template <typename T>
  HsaBuffer<T>* allocate_buffer(size_t size, bool kernarg)
  {
    void* system_ptr = allocate_system_memory(size);
    if (!system_ptr)
      return NULL;
    if (_gpu_local_region.handle == 0)
      return new HsaBuffer<T>(size, static_cast<T*>(system_ptr));

    auto region = kernarg ? _kernarg_region : _gpu_local_region;
    void* agent_ptr = NULL;
    hsa_status_t status = hsa_memory_allocate(region, size, (void**)&agent_ptr);
    if (status != HSA_STATUS_SUCCESS)
    {
      free(system_ptr);
      return NULL;
    }

    return new HsaBuffer<T>(size, static_cast<T*>(system_ptr),
                            static_cast<T*>(agent_ptr));
  }

  private:
  hsa_region_t _system_region;
  hsa_region_t _kernarg_region;
  hsa_region_t _local_region;
  hsa_region_t _gpu_local_region;

  static hsa_status_t setup_memory_region(hsa_region_t region, void* data)
  {
    hsa_region_segment_t segment_id;
    hsa_region_get_info(region, HSA_REGION_INFO_SEGMENT, &segment_id);
    if (segment_id != HSA_REGION_SEGMENT_GLOBAL)
      return HSA_STATUS_SUCCESS;

    hsa_region_global_flag_t flags;
    bool is_host_accessible = false;
    hsa_region_get_info(region, HSA_REGION_INFO_GLOBAL_FLAGS, &flags);
    hsa_region_get_info(region,
                        (hsa_region_info_t)HSA_AMD_REGION_INFO_HOST_ACCESSIBLE,
                        &is_host_accessible);

    HsaMemory* self = static_cast<HsaMemory*>(data);

    if (flags & HSA_REGION_GLOBAL_FLAG_KERNARG)
      self->set_kernarg_region(region);
    if (flags & HSA_REGION_GLOBAL_FLAG_FINE_GRAINED)
      self->set_system_region(region);
    if (flags & HSA_REGION_GLOBAL_FLAG_COARSE_GRAINED && is_host_accessible)
      self->set_local_region(region);
    if (flags & HSA_REGION_GLOBAL_FLAG_COARSE_GRAINED && !is_host_accessible)
      self->set_gpu_local_region(region);

    return HSA_STATUS_SUCCESS;
  }
};
