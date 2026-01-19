cmake_minimum_required(VERSION 3.10)

project(ECC LANGUAGES C CXX)

add_executable(
  ecc
  include/ecc/io.h
  include/ecc/lexer.hpp
  src/io.c
  src/lexer.cpp
  src/main.cpp
)

target_include_directories(ecc PRIVATE include)
