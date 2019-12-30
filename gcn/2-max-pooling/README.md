1. Fetch third-party dependencies:

```
mkdir include
cd include
curl -O https://raw.githubusercontent.com/vsrad/radeon-asm-tools/77377590a34b4ce00755a55a3673cfedb6c6c8ed/Example/VectorAddProjectExample/gfx9/include/common.inc
curl -O https://raw.githubusercontent.com/vsrad/radeon-asm-tools/77377590a34b4ce00755a55a3673cfedb6c6c8ed/Example/VectorAddProjectExample/gfx9/include/inst_wrappers.inc
curl -O https://raw.githubusercontent.com/vsrad/radeon-asm-tools/77377590a34b4ce00755a55a3673cfedb6c6c8ed/Example/VectorAddProjectExample/gfx9/include/gpr_alloc.inc
curl -O https://raw.githubusercontent.com/vsrad/debug-plug-hsa-intercept/68ef9aae617f5711ae6a5ad651c2086589f75757/tests/fixtures/breakpoint.pl
```

2. Compile the kernel and the host executable:

```
make
```

3. Run it:

```
./max_pooling
```

## Debugging

1. Download and build [libplugintercept.so](https://github.com/vsrad/debug-plug-hsa-intercept) on your remote machine

2. Open *MaxPooling.sln* in Visual Studio with [Radeon Asm Tools](https://github.com/vsrad/radeon-asm-tools) installed

3. Edit *debug.sh*: replace `HSA_TOOLS_LIB` export with the path on the remote machine

4. Tailor the Visual Studio profile (*Tools -> RAD Debug -> Options*) to your remote machine
