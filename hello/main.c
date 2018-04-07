#include "io.h"
#include "cl_utils.h"

int main() {
    char* kernel_src;

    kernel_src = read_file("square.cl");
    if (!kernel_src) DIE("Unable to load kernel source\n");

    cl_int err;

    cl_device_id device = cl_gpu_device(cl2_platform());
    cl_context context = clCreateContext(NULL, 1, &device, NULL, NULL, &err); CHK_CL_ERR(err);
    cl_command_queue commands = clCreateCommandQueueWithProperties(context, device, (cl_queue_properties[]) { 0 }, &err); CHK_CL_ERR(err);
    cl_program program = cl_program_from_src(context, device, (const char **) &kernel_src);
    cl_kernel kernel = clCreateKernel(program, "square", &err); CHK_CL_ERR(err);

    unsigned int count = 2048;
    float data[count];
    for (int i = 0; i < count; i++) data[i] = i;

    cl_mem input = clCreateBuffer(context,  CL_MEM_READ_ONLY,  sizeof(float) * count, NULL, &err); CHK_CL_ERR(err);
    cl_mem output = clCreateBuffer(context, CL_MEM_WRITE_ONLY, sizeof(float) * count, NULL, &err); CHK_CL_ERR(err);

    HANDLE_CL_ERR(clEnqueueWriteBuffer(commands, input, CL_TRUE, 0, sizeof(float) * count, data, 0, NULL, NULL));
    HANDLE_CL_ERR(clSetKernelArg(kernel, 0, sizeof(cl_mem), &input));
    HANDLE_CL_ERR(clSetKernelArg(kernel, 1, sizeof(cl_mem), &output));
    HANDLE_CL_ERR(clSetKernelArg(kernel, 2, sizeof(unsigned int), &count));

    size_t global_work_size = count;
    size_t local_work_size;
    HANDLE_CL_ERR(clGetKernelWorkGroupInfo(kernel, device, CL_KERNEL_WORK_GROUP_SIZE,
        sizeof(local_work_size), &local_work_size, NULL));
    printf("Global work size: %zu, local work size: %zu", global_work_size, local_work_size);

    HANDLE_CL_ERR(clEnqueueNDRangeKernel(commands, kernel, 1, NULL, &global_work_size, &local_work_size, 0, NULL, NULL));
    HANDLE_CL_ERR(clFinish(commands));

    float results[count];
    HANDLE_CL_ERR(clEnqueueReadBuffer(commands, output, CL_TRUE, 0, sizeof(float) * count, results, 0, NULL, NULL ));

    clReleaseMemObject(input);
    clReleaseMemObject(output);
    clReleaseProgram(program);
    clReleaseKernel(kernel);
    clReleaseCommandQueue(commands);
    clReleaseContext(context);

    /* Validate results */
    for (int i = 0; i < count; i++) {
        float expected = data[i] * data[i];
        if (results[i] != expected) {
            printf("Result #%i is invalid: expected %.1f, got %.1f", i, expected, results[i]);
        }
    }
}
