cmake_minimum_required(VERSION 3.30)
project(
  ametokei
  VERSION 0.1.0
  LANGUAGES CXX)

if(CMAKE_CURRENT_SOURCE_DIR STREQUAL CMAKE_CURRENT_BINARY_DIR AND NOT MSVC_IDE)
  message(
    FATAL_ERROR
      "In-source builds are not allowed.
Please create a directory and run cmake from there, passing the path
to this source directory as the last argument.
This process created the file `CMakeCache.txt' and the directory `CMakeFiles'.
Please delete them.")
endif()

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(AMETOKEI_SRCS ${CMAKE_CURRENT_SOURCE_DIR}/ametokei/Main.cc
                  ${CMAKE_CURRENT_SOURCE_DIR}/ametokei/nao/StringRef.cc)

add_executable(ametokei ${AMETOKEI_SRCS})
target_include_directories(ametokei PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/ametokei
                                            ${CMAKE_CURRENT_SOURCE_DIR}/include)
