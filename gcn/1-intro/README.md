Download dependencies:

```
mkdir vendor && curl https://raw.githubusercontent.com/dtschump/CImg/master/CImg.h -o vendor/CImg.h
```

Compile the kernel and the host executable:

```
make
```

## References

* https://github.com/ROCm-Developer-Tools/LLVM-AMDGPU-Assembler-Extra/tree/master/examples/asm-kernel
* https://github.com/ROCm-Developer-Tools/LLVM-AMDGPU-Assembler-Extra/issues/20
* https://gist.github.com/adityaatluri/d8ed1f66024840f8cb982ef585b7c31f
