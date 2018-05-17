__kernel void tiled(const __global float* A,
                    const __global float* B,
                    __global float* C,
                    const uint M,
                    const uint N,
                    const uint P) {
    /* This kernel takes advantage of the __local memory shared between
     * all work items in a work group.
     *
     * Consider the following situation:
     *  
     * | --- | --- |     | --- | --- | --- |     | --- | --- | --- |
     * |  X  |  X  |     |  X  |     |     |     |  X  |     |     |
     * | --- | --- |  *  | --- | --- | --- |  =  | --- | --- | --- |
     * |     |     |     |  X  |     |     |     |     |     |     |   ( M / TILE_SIZE rows )
     * | --- | --- |     | --- | --- | --- |     | --- | --- | --- |
     * |     |     |                             |     |     |     |
     * | --- | --- |                             | --- | --- | --- |
     *
     *                                          ( P / TILE_SIZE cols )
     *
     * The cells are not individual elements, but rather, _square_ tiles the size of
     * the number of work items in a group. This requires matrix dimensions to be
     * divisible by that number; we take care of that by going through
     * ceil(N / TILE_SIZE) sets of tiles (2 in the illustration above)
     * and setting elements with indexes falling outside of matrix dimensions to 0. */
    const size_t tile_num = (uint) ceil((float) ((float) N / (float) TILE_SIZE));

    /* A work group fills in a single tile in the C matrix */
    const size_t tile_row = get_group_id(0);
    const size_t tile_col = get_group_id(1);

    /* Each work item computes a single element inside that tile */
    const size_t row = get_local_id(0);
    const size_t col = get_local_id(1);

    /* The element is accumulated through iterations on A and B matrix tiles */
    float c_acc = 0.0f;
    
    /* The tiles currently being iterated on are shared within a work group:
     * each work item loads a single element from each input matrix, and once
     * memory reads are synchronized, every work item has access to
     * all values with these tiles. */
    __local float current_a_tile[TILE_SIZE][TILE_SIZE];
    __local float current_b_tile[TILE_SIZE][TILE_SIZE];

    for (size_t tile = 0; tile < tile_num; tile++) {
        const size_t a_i_row = (tile_row * TILE_SIZE) + row;
        const size_t a_i_col = (tile * TILE_SIZE) + col;

        const size_t b_i_row = (tile * TILE_SIZE) + row;
        const size_t b_i_col = (tile_col * TILE_SIZE) + col;

        /* If the dimensions are not divisible by the number of work items (TILE_SIZE),
         * we may encounter elements that are outside the matrix -- treat those as 0s. */
        if (a_i_row >= M || a_i_col >= N) current_a_tile[row][col] = 0.0f;
        else current_a_tile[row][col] = A[a_i_row * N + a_i_col];

        if (b_i_row >= N || b_i_col >= P) current_b_tile[row][col] = 0.0f;
        else current_b_tile[row][col] = B[b_i_row * P + b_i_col];

        /* After synchronization, we'll have access to all elements in current A and B tiles */
        barrier(CLK_LOCAL_MEM_FENCE);

        for (size_t n = 0; n < TILE_SIZE; n++)
            c_acc += current_a_tile[row][n] * current_b_tile[n][col];

        /* Wait for all work items to finish reading current tiles before loading the next ones */
        barrier(CLK_LOCAL_MEM_FENCE);
    }

    /* Remember that there might be more work items than there are elements in edge tiles */
    const size_t result_row = get_global_id(0);
    const size_t result_col = get_global_id(1);
    const size_t result_index = (tile_row * TILE_SIZE * P) + (tile_col * TILE_SIZE) + (row * P) + col;
    if (result_row < M && result_col < P) C[result_index] = c_acc;
}
