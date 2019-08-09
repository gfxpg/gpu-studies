cmake_minimum_required(VERSION 3.14)

find_package(OpenMP REQUIRED)

add_executable(ray-tracing
  main.cpp
)

target_link_libraries(ray-tracing PRIVATE OpenMP::OpenMP_CXX)
