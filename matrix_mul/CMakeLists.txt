cmake_minimum_required(VERSION 3.10)
project(matrix_mul)

set(CMAKE_CXX_STANDARD 11)

add_executable(matrix_mul main.c io.h)

find_package(OpenCL REQUIRED)
target_link_libraries(matrix_mul OpenCL::OpenCL m)