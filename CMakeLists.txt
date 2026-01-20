cmake_minimum_required(VERSION 3.12)

project(ECC LANGUAGES CXX)

add_executable(
  ecc
  include/ecc/ast.hpp
  include/ecc/lexer.hpp
  include/ecc/parser.hpp
  src/ast.cpp
  src/lexer.cpp
  src/main.cpp
  src/parser.cpp
)

target_include_directories(ecc PRIVATE include)

set_property(TARGET ecc PROPERTY CXX_STANDARD 20)
set_property(TARGET ecc PROPERTY CXX_STANDARD_REQUIRED ON)
set_property(TARGET ecc PROPERTY CXX_EXTENSIONS OFF)
