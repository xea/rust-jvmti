cmake_minimum_required(VERSION 3.3)

project (cvoid)
set(CMAKE_BUILD_TYPE Release)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)
link_directories(target/debug)
find_library(jvmti jvmti target/debug)

#add_library(cvoid SHARED src/test.c)
add_executable(cmain src/test.c)

target_link_libraries(cmain jvmti)
