#define COMP_PER_THREAD 8
#define GROUP_COLS 8
#define TILE_ROWS (COMP_PER_THREAD * GROUP_COLS)
#define TILE_WIDTH (TILE_SIZE / 4)

/* Assuming input matrices are MxN (A), NxP (B):
 * global x size is M / 8, because each thread computes 8 values (see COMP_PER_THREAD)
 * global y size is P / 4, because we're using wide loads (float4)
 * local x size is COMP_PER_THREAD
 * local y size is GROUP_COLS
 */

__kernel void subgroups(const __global float4* A,
                        const __global float4* B,
                        __global float4* C,
                        const uint M,
                        const uint N,
                        const uint P) {
    const size_t n_wide = N / 4;
    const size_t p_wide = P / 4;

    const size_t tile_row = get_group_id(0);
    const size_t tile_col = get_group_id(1);
    const size_t row = get_local_id(0);
    const size_t col = get_local_id(1);

    float4 c_tile[] = { 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f };

    __global float4* C_tile = C + row + (tile_col * TILE_WIDTH) + (tile_row * TILE_ROWS + col * GROUP_COLS) * p_wide;
    __global float4* A_tile = A + row + (tile_row * TILE_ROWS + col * GROUP_COLS) * n_wide;
    __global float4* B_tile = B + row + (tile_col * TILE_WIDTH);

    for (size_t tile = 0; tile < N / TILE_SIZE; tile++) {
        const float4 a_tile_rows[] = {
            A_tile[0], A_tile[n_wide], A_tile[2 * n_wide], A_tile[3 * n_wide], A_tile[4 * n_wide], A_tile[5 * n_wide], A_tile[6 * n_wide], A_tile[7 * n_wide]
        };
        A_tile += TILE_WIDTH;
        
#pragma unroll
        for (size_t section = 0; section < TILE_WIDTH; section++) {
            /* TODO: When using an array, shuffle always loads data from the first thread, no matter what `section` it is.
             * Why? Need to check out assembly */
            const float4 a0 = intel_sub_group_shuffle(a_tile_rows[0], section);
            const float4 a1 = intel_sub_group_shuffle(a_tile_rows[1], section);
            const float4 a2 = intel_sub_group_shuffle(a_tile_rows[2], section);
            const float4 a3 = intel_sub_group_shuffle(a_tile_rows[3], section);
            const float4 a4 = intel_sub_group_shuffle(a_tile_rows[4], section);
            const float4 a5 = intel_sub_group_shuffle(a_tile_rows[5], section);
            const float4 a6 = intel_sub_group_shuffle(a_tile_rows[6], section);
            const float4 a7 = intel_sub_group_shuffle(a_tile_rows[7], section);

            const float4 b_section[4] = {
                B_tile[0], B_tile[1 * p_wide], B_tile[2 * p_wide], B_tile[3 * p_wide]
            };
            B_tile += p_wide * 4;

#pragma unroll
            for (size_t c = 0; c < 4; c++) c_tile[0] += b_section[c] * a0[c];
#pragma unroll
            for (size_t c = 0; c < 4; c++) c_tile[1] += b_section[c] * a1[c];
#pragma unroll
            for (size_t c = 0; c < 4; c++) c_tile[2] += b_section[c] * a2[c];
#pragma unroll
            for (size_t c = 0; c < 4; c++) c_tile[3] += b_section[c] * a3[c];
#pragma unroll
            for (size_t c = 0; c < 4; c++) c_tile[4] += b_section[c] * a4[c];
#pragma unroll
            for (size_t c = 0; c < 4; c++) c_tile[5] += b_section[c] * a5[c];
#pragma unroll
            for (size_t c = 0; c < 4; c++) c_tile[6] += b_section[c] * a6[c];
#pragma unroll
            for (size_t c = 0; c < 4; c++) c_tile[7] += b_section[c] * a7[c];
        }
    }

    for (size_t r = 0; r < TILE_WIDTH; r++) C_tile[r * p_wide] = c_tile[r];
}
