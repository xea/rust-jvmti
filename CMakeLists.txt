cmake_minimum_required(VERSION 3.3)

project (cvoid)
set(CMAKE_BUILD_TYPE Debug)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)
link_directories(target/debug)
find_library(jvmti jvmti target/debug)

# C Agent is the native agent that forwards out to Rust
add_library(cagent SHARED src/cagent.c)

# C Lib is the shared library that can accept calls from Rust
add_library(clib SHARED src/clib.c)
add_library(clib_static STATIC src/clib.c)

# C Main is an executable testbed framework for the Rust code
add_executable(cmain src/cmain.c)

target_link_libraries(cmain jvmti)
target_link_libraries(cagent jvmti)
