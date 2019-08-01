cmake_minimum_required(VERSION 3.14)
project(ray-tracing VERSION 0.1.0)

add_subdirectory(extern)
add_subdirectory(src)

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
