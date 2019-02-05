cmake_minimum_required(VERSION 3.10)
project(hello)

set(CMAKE_CXX_STANDARD 11)

add_executable(hello main.c cl_utils.h cl_utils.c)

find_package(OpenCL REQUIRED)
target_link_libraries(hello OpenCL::OpenCL)