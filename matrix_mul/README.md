# Matrix multiplication

## Prerequisites

You need OpenCL development files -- headers and libraries -- installed on your system
to build the program, as well as Python3 and Numpy to run the matrix generation script
(see below for more details).

## Building

`cmake . && make`

## Running

The program reads input matrices from `matrix_a` and `matrix_b` files, multiplies them,
and compares the result with the matrix read from `matrix_c`. You can generate these files
with the included `mkmatrices` script (run `./mkmatrices -h` for usage instructions).

`matrix_mul` expects four arguments: the target OpenCL platform and matrix dimensions.
The latter should match the dimensions you've passed to `mkmatrices`; to determine
the former, run `clinfo` and pick your GPU's platform
(note that `Device OpenCL C Version` should be at least OpenGL 1.2).

## Gotchas

On Intel's iGPUs, the kernel may not be executed over the full global work range,
which would result in validation errors (some of the values ending up being 0).
An execution counter atomically incremented in the kernel would show that the
number of times the kernel is ran is indeed less than the global work size.

In my case, this was related to GPU hang checks executed by the driver. Run
`dmesg | grep 'timed out'`, and if you see messages like 
`asynchronous wait on fence i915:[...] timed out`, then that's most likely the problem.

The solution is disabling the checks by running
`echo -n 0 > /sys/module/i915/parameters/enable_hangcheck`. Note that with the check
disabled, if the kernel really hangs you won't be able to do anything but reboot.

## Profiling results

### Intel

Running on an [Intel Core i5-8250U](https://ark.intel.com/products/124967)-powered laptop.

clinfo:

```
Platform Name                                   Intel Gen OCL Driver
Device Name                                     Intel(R) HD Graphics Kabylake Desktop GT1.5
Device Version                                  OpenCL 2.0 beignet 1.3
Driver Version                                  1.3
```

Results:

```
===
=== tiled.cl
===
Global work size: 5000 x 5000, local work size: 20 x 20
Kernel execution time is 13035.612320 [ms]
Time from enqueueing to execution is 11.710560 [ms]
===
=== simple.cl
===
Global work size: 5000 x 5000, local work size: 20 x 20
Kernel execution time is 41476.164800 [ms]
Time from enqueueing to execution is 0.092880 [ms]
```

### NVIDIA

Running on NVIDIA Quadro P4000 at [Paperspace](https://www.paperspace.com/&R=QX3FBZI) cloud.

clinfo:

```
Platform Name                                   NVIDIA CUDA
Device Name                                     Quadro P4000
Device Version                                  OpenCL 1.2 CUDA
Driver Version                                  384.111
```

Results:

```
===
=== tiled.cl
===
Global work size: 5000 x 5000, local work size: 20 x 20
Kernel execution time is 910.439168 [ms]
Time from enqueueing to execution is 0.008960 [ms]
===
=== simple.cl
===
Global work size: 5000 x 5000, local work size: 20 x 20
Kernel execution time is 5226.043392 [ms]
Time from enqueueing to execution is 0.007936 [ms]
```