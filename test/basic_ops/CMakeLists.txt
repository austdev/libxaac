cmake_minimum_required(VERSION 3.10)

project(BasicOpsDemo C)

# Set C standard
set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Enable warnings
add_compile_options(-Wall)

# Create executable
add_executable(main main.c)