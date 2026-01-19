cmake_minimum_required(VERSION 3.10)
project(ECC)
add_executable(ecc src/io.c src/lexer.c src/main.c)
target_include_directories(ecc PRIVATE include)
