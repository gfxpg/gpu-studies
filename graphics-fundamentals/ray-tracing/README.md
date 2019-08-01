# Toy Ray Tracer

## Prerequisites

* CMake 3.14+
* GCC

The project does not compile with Clang because it lacks
(non-standard) `constexpr` definitions of trigonometric functions.

## Building and Running Tests

```sh
mkdir build
cd build
cmake ..
make && make test
```
