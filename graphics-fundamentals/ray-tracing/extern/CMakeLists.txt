cmake_minimum_required(VERSION 3.14)
include(FetchContent)

# Based on https://manenko.com/2018/09/30/manage-external-dependencies-with-cmake.html

FetchContent_Declare(
  extern_catch2
  GIT_REPOSITORY https://github.com/catchorg/Catch2.git
  GIT_TAG        v2.9.1
)
FetchContent_GetProperties(extern_catch2)
if(NOT extern_catch2_POPULATED)
  FetchContent_Populate(extern_catch2)
  add_subdirectory(${extern_catch2_SOURCE_DIR} ${extern_catch2_BINARY_DIR})
  set_property(GLOBAL PROPERTY CATCH2_MODULE_PATH ${extern_catch2_SOURCE_DIR}/contrib)
endif()
