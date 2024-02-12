# quic_over_socket_api

An experimental library which is supposed to be able to replace udp sockets with quic,
without having to change the source code of a project.

## CMake Integration
```cmake
cmake_minimum_required(VERSION 3.0)
project(YourCppApplication)

# Include the Rust project
add_subdirectory(path/to/quic_over_socket_api)

# Your C++ executable
add_executable(${PROJECT_NAME} main.cpp)

# Link the Rust static library with your C++ application
target_link_libraries(${PROJECT_NAME} quic_over_socket_api_static)
```