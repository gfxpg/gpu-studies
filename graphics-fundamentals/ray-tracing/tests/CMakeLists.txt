cmake_minimum_required(VERSION 3.14)

add_executable(tests
  main.cpp
  test_vec3.cpp
)

target_link_libraries(tests Catch2::Catch2)

get_property(CMAKE_MODULE_PATH GLOBAL PROPERTY CATCH2_MODULE_PATH)

include(CTest)
include(Catch)
catch_discover_tests(tests)

add_test(NAME tests COMMAND tests)
