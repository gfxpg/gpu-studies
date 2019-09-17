.amdgcn_target "amdgcn-amd-amdhsa--gfx900"

.text
.globl hello_world
.p2align 8
.type hello_world,@function
hello_world:
  ; http://www.hsafoundation.com/html/Content/Runtime/Topics/02_Core/hsa_kernel_dispatch_packet_t.htm
  ; 0x4 = workgroup size X (16 bits)
  ; 0x6 = workgroup size Y (16 bits)
  ; 0x14 = grid size X
  ; 0x18 = grid size Y
  ; s[0:1] is the dispatch packet pointer
  ; s[2:3] is the kernarg pointer
  ; s4 is the workgroup X index
  ; s5 is the workgroup Y index
  ; v1 is the workitem X index
  ; v2 is the workitem Y index

  s_load_dword s6, s[4:5], 0x4 ; s6 <- workgroup size X
  s_load_dword s7, s[4:5], 0x6 ; s7 <- workgroup size Y
  s_waitcnt lgkmcnt(0)
  s_and_b32 s6, s6, 0xffff ; keep only the lower 16 bits
  s_and_b32 s7, s7, 0xffff

  s_mul_i32 s4, s4, s6 ; s4 <- workgroup index X * workgroup size X
  s_mul_i32 s5, s5, s7 ; s5 <- workgroup index Y * workgroup size Y

  v_add_u32 v0, s4, v0 ; v0 <- global X id
  v_add_u32 v1, s5, v1 ; v1 <- global Y id

  s_load_dword s6, s[4:5], 0x18 ; s6 <- grid size Y
  s_waitcnt lgkmcnt(0)

  ; index = 3 * (x * grid size y + y)
  v_mul_lo_u32 v0, v0, s6 ; x * grid size y
  v_add_u32 v0, v0, v1 ; + y
  v_mul_lo_u32 v0, v0, 3

  s_load_dwordx2 s[2:3], s[2:3] 0x0 ; buffer pointer
  s_waitcnt lgkmcnt(0)

  ; ..? magic

  flat_store_dword v[1:2], v0
  s_endpgm
.Lfunc_end0:
  .size   hello_world, .Lfunc_end0-hello_world

.rodata
.p2align 6
.amdhsa_kernel hello_world
  .amdhsa_user_sgpr_kernarg_segment_ptr 1
  .amdhsa_user_sgpr_dispatch_ptr 1
  .amdhsa_system_sgpr_workgroup_id_x 1
  .amdhsa_system_sgpr_workgroup_id_y 1
  .amdhsa_system_vgpr_workitem_id 1
  .amdhsa_next_free_vgpr .amdgcn.next_free_vgpr
  .amdhsa_next_free_sgpr .amdgcn.next_free_sgpr
.end_amdhsa_kernel
