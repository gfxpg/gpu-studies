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
     * the number of work items in a group. This requires the dimensions of matrices
     * to be divisible by that number, but that's taken care of on the host.
     *
     * In total, we need to go through (N / TILE_SIZE) sets of tiles -- in
     * the illustration above, tile_num is 2. */
    const size_t tile_num = N / TILE_SIZE;

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

    const size_t a_tile_row_stride = TILE_SIZE * N;
    const size_t b_tile_row_stride = TILE_SIZE * P;

    for (size_t tile = 0; tile < tile_num; tile++) {
        const size_t a_i = (a_tile_row_stride * tile_row) + (tile * TILE_SIZE) + (row * N) + col;
        const size_t b_i = (b_tile_row_stride * tile) + (tile_col * TILE_SIZE) + (row * P) + col;
        current_a_tile[row][col] = A[a_i];
        current_b_tile[row][col] = B[b_i];
    
        /* After synchronization, we'll have access to all elements in current A and B tiles */
        barrier(CLK_LOCAL_MEM_FENCE);

        for (size_t n = 0; n < TILE_SIZE; n++)
            c_acc += current_a_tile[row][n] * current_b_tile[n][col];

        /* Wait for all work items to finish reading current tiles before loading the next ones */
        barrier(CLK_LOCAL_MEM_FENCE);
    }

    C[(tile_row * a_tile_row_stride) + (tile_col * TILE_SIZE) + (row * P) + col] = c_acc;
}
