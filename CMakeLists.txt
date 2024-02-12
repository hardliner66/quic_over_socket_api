cmake_minimum_required(VERSION 3.0)
project(quic_over_socket_api)

# Define variables for the Rust library output
set(RUST_LIB_NAME quic_over_socket_api)
set(RUST_TARGET_DIR ${CMAKE_CURRENT_SOURCE_DIR}/target/release)
set(RUST_LIB_FILE ${RUST_TARGET_DIR}/lib${RUST_LIB_NAME}.a)

# Custom command to build the Rust library using Cargo
add_custom_target(
    ${RUST_LIB_NAME} ALL
    COMMAND cargo build --release
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Compiling Rust library ${RUST_LIB_NAME}"
)

# Define the library to be linked with the C++ project
add_library(quic_over_socket_api_static STATIC IMPORTED GLOBAL)
add_dependencies(quic_over_socket_api_static ${RUST_LIB_NAME})
set_target_properties(quic_over_socket_api_static PROPERTIES
    IMPORTED_LOCATION ${RUST_LIB_FILE}
    INTERFACE_INCLUDE_DIRECTORIES ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# Optionally, specify include directories if your Rust library provides a C header
