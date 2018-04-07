/* Various OpenCL routines */

#ifndef CL_UTILS_H
#define CL_UTILS_H

#include <CL/cl.h>

#include "io.h"

cl_program cl_program_from_src(cl_context, cl_device_id, const char **);
cl_device_id cl_gpu_device(cl_platform_id);
cl_platform_id cl2_platform();

#endif //CL_UTILS_H
