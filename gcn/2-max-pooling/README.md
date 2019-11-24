1. Fetch third-party dependencies:

```
mkdir include
cd include
curl -O https://raw.githubusercontent.com/vsrad/radeon-asm-tools/77377590a34b4ce00755a55a3673cfedb6c6c8ed/Example/VectorAddProjectExample/gfx9/include/common.inc
curl -O https://raw.githubusercontent.com/vsrad/radeon-asm-tools/77377590a34b4ce00755a55a3673cfedb6c6c8ed/Example/VectorAddProjectExample/gfx9/include/inst_wrappers.inc
curl -O https://raw.githubusercontent.com/vsrad/radeon-asm-tools/77377590a34b4ce00755a55a3673cfedb6c6c8ed/Example/VectorAddProjectExample/gfx9/include/gpr_alloc.inc
```

2. Compile the kernel and the host executable:

```
make
```
