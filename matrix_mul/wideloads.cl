__kernel void wideloads(const __global float4* A,
                        const __global float4* B,
                        __global float4* C,
                        const uint M,
                        const uint N,
                        const uint P) {
    /* This kernel is similar to tiled.cl. The difference that makes it perform better
     * is having each work item calculate four instead of one elements in a row
     * (the number of rows, as well as the tile size, must be a multiple of four).
     * A single load can fetch four values using wide data types (float4), which
     * uses the available memory bandwidth more effectively. */
    #define TILE_SIZE 20
    const size_t tile_num = N / TILE_SIZE;

    const size_t tile_row = get_group_id(0);
    const size_t tile_col = get_group_id(1);
    const size_t row = get_local_id(0);
    const size_t col = get_local_id(1);

    float4 c_acc = { 0.0f, 0.0f, 0.0f, 0.0f };

    /* Mind that each column is actually four separate values */
    __local float4 current_a_tile[TILE_SIZE][TILE_SIZE / 4];
    __local float4 current_b_tile[TILE_SIZE][TILE_SIZE / 4];

    const size_t a_tile_row_stride = TILE_SIZE * N / 4;
    const size_t b_tile_row_stride = TILE_SIZE * P / 4;

    for (size_t tile = 0; tile < tile_num; tile++) {
        const size_t a_i = (a_tile_row_stride * tile_row) + (tile * TILE_SIZE / 4) + (row * N / 4) + col;
        const size_t b_i = (b_tile_row_stride * tile) + (tile_col * TILE_SIZE / 4) + (row * P / 4) + col;
        current_a_tile[row][col] = A[a_i];
        current_b_tile[row][col] = B[b_i];

        barrier(CLK_LOCAL_MEM_FENCE);

        float4 a_section, b_section;

        for (size_t section = 0; section < (TILE_SIZE / 4); section++) {
            a_section = current_a_tile[row][section];
            float* a_value_ptr = &a_section;

            __attribute__((opencl_unroll_hint(4)))
            for (size_t section_el = 0; section_el < 4; section_el++) {
                b_section = current_b_tile[(section * 4) + section_el][col];
                c_acc += *a_value_ptr * b_section;
                a_value_ptr++;
          }
        }

        barrier(CLK_LOCAL_MEM_FENCE);
    }

    C[(tile_row * a_tile_row_stride) + (tile_col * TILE_SIZE / 4) + (row * P / 4) + col] = c_acc;
}
