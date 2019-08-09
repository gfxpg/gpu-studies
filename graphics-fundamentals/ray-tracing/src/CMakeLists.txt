cmake_minimum_required(VERSION 3.14)

find_package(PNGwriter 0.7.0 REQUIRED)
find_package(OpenMP REQUIRED)

add_executable(ray-tracing
  main.cpp
)

target_link_libraries(ray-tracing PRIVATE PNGwriter::PNGwriter OpenMP::OpenMP_CXX)
