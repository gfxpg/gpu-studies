__kernel void pad_cols(const __global float* A,
                       __global float* AT,
                       const uint M,
                       const uint N) {
    const uint row_i = get_global_id(0);
    const uint col_i = get_global_id(1);
    const uint out_of_bounds = (N % TILE_SIZE == 0) ? 0 : TILE_SIZE - (N % TILE_SIZE);

    if (row_i >= M)
        return;
    if (col_i >= N)
        AT[(row_i * (N + out_of_bounds)) + col_i] = 0.0f;
    else
        AT[(row_i * (N + out_of_bounds)) + col_i] = A[(row_i * N) + col_i];
}
