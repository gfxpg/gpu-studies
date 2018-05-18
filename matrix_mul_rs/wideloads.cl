__kernel void wideloads(const __global float4* A,
                        const __global float4* B,
                        __global float* C,
                        const uint M,
                        const uint N,
                        const uint P) {
    /* This kernel is similar to tiled.cl. The difference that makes it perform better
     * is having each work item calculate four instead of one elements in a row
     * (the number of rows, as well as the tile size, is required to be a multiple of
     * four, which is satisfied by having a padding kernel run on the input matrices).
     *
     * The reason that performs better is that with wide data types (float4),
     * a single load can fetch four instead of one values, which uses the available
     * memory bandwidth more effectively. */
    const size_t tile_num = (uint) ceil((float) ((float) N / TILE_SIZE));
    const size_t n_wide = tile_num * TILE_SIZE / 4;
    const size_t p_wide = ((uint) ceil((float) ((float) P / TILE_SIZE))) * TILE_SIZE / 4;
    const size_t out_of_bounds = P % TILE_SIZE;

    const size_t tile_row = get_group_id(0);
    const size_t tile_col = get_group_id(1);
    const size_t row = get_local_id(0);
    const size_t col = get_local_id(1);

    float4 c_acc = { 0.0f, 0.0f, 0.0f, 0.0f };

    /* Mind that each column is actually four separate values */
    __local float4 current_a_tile[TILE_SIZE][TILE_SIZE / 4];
    __local float4 current_b_tile[TILE_SIZE][TILE_SIZE / 4];

    for (size_t tile = 0; tile < tile_num; tile++) {
        const size_t a_i_row = (tile_row * TILE_SIZE) + row;
        const size_t a_i_col = (tile * TILE_SIZE / 4) + col;

        const size_t b_i_row = (tile * TILE_SIZE) + row;
        const size_t b_i_col = (tile_col * TILE_SIZE / 4) + col;

        /* If the vertical dimension is not divisible by the number of work items (TILE_SIZE),
         * we may encounter elements that are outside the matrix -- treat those as 0s. */
        if (a_i_row >= M) current_a_tile[row][col] = (float4) { 0.0f, 0.0f, 0.0f, 0.0f };
        else current_a_tile[row][col] = A[a_i_row * n_wide + a_i_col];

        if (b_i_row >= N) current_b_tile[row][col] = (float4) { 0.0f, 0.0f, 0.0f, 0.0f };
        else current_b_tile[row][col] = B[b_i_row * p_wide + b_i_col];

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
    
    /* Remember that there might be more work items than there are elements in edge tiles */
    const size_t result_row = get_global_id(0);
    const size_t result_col = get_global_id(1) * 4;
    const size_t result_index = (tile_row * TILE_SIZE * P) + (tile_col * TILE_SIZE) + (row * P) + (col * 4);

    if (result_row >= M || result_col >= P) return;

    switch ((result_col + 4) - P) {
        case 1:
            C[result_index] = c_acc.s0;
            C[result_index + 1] = c_acc.s1;
            C[result_index + 2] = c_acc.s2;
            break;
        case 2:
            C[result_index] = c_acc.s0;
            C[result_index + 1] = c_acc.s1;
            break;
        case 3:
            C[result_index] = c_acc.s0;
            break;
        default:
            C[result_index] = c_acc.s0;
            C[result_index + 1] = c_acc.s1;
            C[result_index + 2] = c_acc.s2;
            C[result_index + 3] = c_acc.s3;
    }
}
