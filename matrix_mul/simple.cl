__kernel void simple(const __global float* A,
                     const __global float* B,
                     __global float* C,
                     const unsigned int M,
                     const unsigned int N,
                     const unsigned int P) {
    int m = get_global_id(0);
    int p = get_global_id(1);

    float acc = 0.0f;
    for (unsigned int n = 0; n < N; n++) {
        acc += A[m * N + n] * B[n * P + p];
    }

    C[m * P + p] = acc;
}
