#include <string.h>

#include "cl_utils.h"

cl_program cl_program_from_src(cl_context context, cl_device_id device, const char ** source) {
    cl_int err;

    cl_program program = clCreateProgramWithSource(context, 1, source, NULL, &err); CHK_CL_ERR(err);

    err = clBuildProgram(program, 0, NULL, "-cl-std=CL2.0", NULL, NULL);
    if (err != CL_SUCCESS) {
        char buffer[2048];
        clGetProgramBuildInfo(program, device, CL_PROGRAM_BUILD_LOG, sizeof(buffer), buffer, NULL);

        fprintf(stderr, "Failed to build the kernel from source, refer to the build log below:\n%s", buffer);
        exit(1);
    }

    return program;
}

cl_device_id cl_gpu_device(cl_platform_id platform_id) {
    cl_device_id device_id;
    HANDLE_CL_ERR(clGetDeviceIDs(platform_id, CL_DEVICE_TYPE_GPU, 1, &device_id, NULL));

    char buffer[64];
    HANDLE_CL_ERR(clGetDeviceInfo(device_id, CL_DEVICE_NAME, sizeof(buffer), &buffer, NULL));
    printf("Device: %s\n", buffer);

    return device_id;
}

cl_platform_id cl2_platform() {
    cl_uint platform_count;
    HANDLE_CL_ERR(clGetPlatformIDs(0, NULL, &platform_count));
    if (platform_count == 0) DIE("No OpenCL platforms found\n");

    cl_platform_id* platform_ids = (cl_platform_id*) malloc(platform_count * sizeof(cl_platform_id));
    HANDLE_CL_ERR(clGetPlatformIDs(platform_count, platform_ids, NULL));

    char buffer[64];
    cl_platform_id target_platform_id = 0;

    printf("Looking for OpenCL 2.0 platforms...\n");
    for (cl_uint i = 0; i < platform_count; i++) {
        HANDLE_CL_ERR(clGetPlatformInfo(platform_ids[i], CL_PLATFORM_VERSION, sizeof(buffer), buffer, NULL));
        if (strncmp(buffer, "OpenCL 2.0", 10) == 0) {
            target_platform_id = platform_ids[i];
            HANDLE_CL_ERR(clGetPlatformInfo(platform_ids[i], CL_PLATFORM_NAME, sizeof(buffer), buffer, NULL));
            printf("Platform: %s\n", buffer);
        }
    }

    if (!target_platform_id) DIE("No suitable platforms found\n");
    return target_platform_id;
}
