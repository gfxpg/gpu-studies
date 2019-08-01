cmake_minimum_required(VERSION 3.14)

find_package(PNGwriter 0.7.0)

add_executable(ray-tracing
  main.cpp
  vec3.hpp
)

target_link_libraries(ray-tracing PRIVATE PNGwriter::PNGwriter)
