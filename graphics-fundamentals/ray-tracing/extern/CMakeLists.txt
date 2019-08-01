cmake_minimum_required(VERSION 3.14)
include(FetchContent)

FetchContent_Declare(
  extern_pngwriter
  GIT_REPOSITORY https://github.com/pngwriter/pngwriter.git
  GIT_TAG 0.7.0
)
FetchContent_GetProperties(extern_pngwriter)
if(NOT extern_pngwriter_POPULATED)
  FetchContent_Populate(extern_pngwriter)
  add_subdirectory(${extern_pngwriter_SOURCE_DIR} ${extern_pngwriter_BINARY_DIR})
  set(extern_pngwriter_INSTALL_DIR "${extern_pngwriter_BINARY_DIR}/install")
  if(NOT EXISTS ${extern_pngwriter_INSTALL_DIR})
    execute_process(
      COMMAND cmake -DCMAKE_INSTALL_PREFIX=${extern_pngwriter_INSTALL_DIR} ${extern_pngwriter_SOURCE_DIR}
      WORKING_DIRECTORY ${extern_pngwriter_BINARY_DIR}
      RESULT_VARIABLE extern_pngwriter_cmake_result
    )
    if(extern_pngwriter_cmake_result)
      message(FATAL_ERROR "Failed PNGwriter configuration")
    endif()
    execute_process(
      COMMAND make -j8
      WORKING_DIRECTORY ${extern_pngwriter_BINARY_DIR}
      RESULT_VARIABLE extern_pngwriter_make_result
    )
    if(extern_pngwriter_make_result)
      message(FATAL_ERROR "Failed PNGwriter build")
    endif()
    execute_process(
      COMMAND make install
      WORKING_DIRECTORY ${extern_pngwriter_BINARY_DIR}
      RESULT_VARIABLE extern_pngwriter_install_result
    )
    if(extern_pngwriter_install_result)
      message(FATAL_ERROR "Failed PNGwriter local installation")
    endif()
  endif()
endif()

set(CMAKE_PREFIX_PATH ${extern_pngwriter_INSTALL_DIR} ${CMAKE_PREFIX_PATH})
find_package(PNGwriter 0.7.0)
