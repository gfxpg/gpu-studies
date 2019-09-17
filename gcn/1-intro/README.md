Download dependencies:

```
mkdir vendor \
  && curl https://raw.githubusercontent.com/ROCm-Developer-Tools/LLVM-AMDGPU-Assembler-Extra/master/examples/common/dispatch.cpp -o vendor/dispatch.cpp \
  && curl https://raw.githubusercontent.com/ROCm-Developer-Tools/LLVM-AMDGPU-Assembler-Extra/master/examples/common/dispatch.hpp -o vendor/dispatch.hpp
```

Create a code object from the assembly file:

```
/opt/rocm/hcc/bin/llvm-mc -mattr=+code-object-v3 -triple amdgcn-amd-amdhsa -mcpu=gfx900 -filetype=obj test.s -o test.o
/opt/rocm/hcc/bin/clang -target amdgcn--amdhsa test.o -o test.co
```

Compile the host executable:

```
/opt/rocm/hcc/bin/clang++ -I/opt/rocm/hsa/include/hsa -std=c++11 test.cpp vendor/dispatch.cpp -L/opt/rocm/lib -lhsa-runtime64 -o test
```

## References

* https://github.com/ROCm-Developer-Tools/LLVM-AMDGPU-Assembler-Extra/tree/master/examples/asm-kernel
* https://github.com/ROCm-Developer-Tools/LLVM-AMDGPU-Assembler-Extra/issues/20
* https://gist.github.com/adityaatluri/d8ed1f66024840f8cb982ef585b7c31f
