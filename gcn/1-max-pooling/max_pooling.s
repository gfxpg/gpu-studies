.include "gpr_alloc.inc"

// IMPORTANT: Doesn't support more than 1 workgroup yet (output size is thus limited by max workgroup size)

.hsa_code_object_version 2,1
.hsa_code_object_isa

.GPR_ALLOC_BEGIN
  kernarg = 0
  gid_x = 2
  .SGPR_ALLOC_FROM 6
  .SGPR_ALLOC base_in, 2
  .SGPR_ALLOC base_out, 2
  .SGPR_ALLOC c_kh
  .SGPR_ALLOC c_kw
  .SGPR_ALLOC c_w
  .SGPR_ALLOC c_outw
  .SGPR_ALLOC iter_h
  .SGPR_ALLOC iter_w

  .VGPR_ALLOC_FROM 0
  .VGPR_ALLOC tid
  .VGPR_ALLOC tidw
  .VGPR_ALLOC addr, 2
  .VGPR_ALLOC max
  .VGPR_ALLOC elt

.GPR_ALLOC_END

.text
.p2align 8
.amdgpu_hsa_kernel max_pooling

max_pooling:
  .amd_kernel_code_t
    is_ptr64 = 1
    enable_sgpr_kernarg_segment_ptr = 1
    enable_sgpr_workgroup_id_x = 1
    kernarg_segment_byte_size = 24
    compute_pgm_rsrc2_user_sgpr = 2
    enable_vgpr_workitem_id = 1 // x, y
    granulated_workitem_vgpr_count = .AUTO_VGPR_GRANULATED_COUNT
    granulated_wavefront_sgpr_count = .AUTO_SGPR_GRANULATED_COUNT
    wavefront_sgpr_count = .AUTO_SGPR_COUNT
    workitem_vgpr_count = .AUTO_VGPR_COUNT
  .end_amd_kernel_code_t

read_kernargs:
  s_load_dwordx2        s[base_in:base_in+1], s[kernarg:kernarg+1], 0
  s_load_dwordx2        s[base_out:base_out+1], s[kernarg:kernarg+1], 8
  s_load_dword          s[c_kh], s[kernarg:kernarg+1], 16
  s_load_dword          s[c_kw], s[kernarg:kernarg+1], 20
  s_load_dword          s[c_w], s[kernarg:kernarg+1], 24
  s_load_dword          s[c_outw], s[kernarg:kernarg+1], 28

NEGATIVE_INFINITY_FP32 = 0xff800000
init_loops:
  s_mov_b64             s[iter_h:iter_w], 0
  v_mov_b32             v[max], NEGATIVE_INFINITY_FP32

wait_kernargs:
  s_waitcnt             0

kh_loop:
kw_loop:
  v_mul_lo_u32          v[addr], v[tid], s[c_w]          // addr = row * c_w
  v_mul_hi_u32          v[addr+1], v[tid], s[c_w]
  v_add_co_u32          v[addr], vcc, v[addr], v[tidw]   // addr = (row * c_w) + col
  v_addc_co_u32         v[addr+1], vcc, v[addr+1], 0, vcc
  v_lshlrev_b64         v[addr:addr+1], 2, v[addr:addr+1] // addr = ((row * c_w) + col) * sizeof(float)
  v_add_co_u32          v[addr], vcc, s[base_in], v[addr]  // addr = base + ((row * c_w) + col) * sizeof(float)
  v_mov_b32             v[elt], s[base_in+1]
  v_addc_co_u32         v[addr+1], vcc, v[elt], v[addr+1], vcc

  // max = max > elt ? max : elt
  flat_load_dword       v[elt], v[addr:addr+1]
  s_waitcnt             0
  v_max_f32             v[max], v[max], v[elt]

kw_loop_next:
  v_add_u32             v[tidw], v[tidw], 1 // col += 1
  s_add_u32             s[iter_w], s[iter_w], 1
  s_cmp_eq_u32          s[iter_w], s[c_kw]
  s_cbranch_scc0        kw_loop

kw_loop_end:
  v_sub_u32             v[tidw], v[tidw], s[c_kw] // col -= kw
  s_mov_b32             s[iter_w], 0

kh_loop_next:
  v_add_u32             v[tid], v[tid], 1 // row += 1
  s_add_u32             s[iter_h], s[iter_h], 1
  s_cmp_eq_u32          s[iter_h], s[c_kh]
  s_cbranch_scc0        kh_loop

kh_loop_end:
  v_sub_u32             v[tid], v[tid], s[c_kh] // col -= kh

save_result:
  v_mul_lo_u32          v[addr], v[tid], s[c_outw]          // addr = row * c_outw
  v_mul_hi_u32          v[addr+1], v[tid], s[c_outw]
  v_add_co_u32          v[addr], vcc, v[addr], v[tidw]      // addr = (row * c_outw) + col
  v_addc_co_u32         v[addr+1], vcc, v[addr+1], 0, vcc
  v_lshlrev_b64         v[addr:addr+1], 2, v[addr:addr+1]   // addr = ((row * c_outw) + col) * sizeof(float)
  v_add_co_u32          v[addr], vcc, s[base_out], v[addr]  // addr = base + ((row * c_outw) + col) * sizeof(float)
  v_mov_b32             v[elt], s[base_out+1]
  v_addc_co_u32         v[addr+1], vcc, v[elt], v[addr+1], vcc

  flat_store_dword      v[addr:addr+1], v[max]

  s_endpgm
