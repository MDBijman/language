cmake_minimum_required(VERSION 3.2)
project(VM C CXX)
include(CheckLanguage)

set(CMAKE_CXX_STANDARD 17)

# Maybe not use glob?
# Own sources
file(GLOB_RECURSE SOURCES src/*.cpp)
# Common sources
file(GLOB_RECURSE COMMON_SOURCES ${Language_SOURCE_DIR}/common/*.cpp)
list(APPEND SOURCES ${COMMON_SOURCES})

# Choose the right implementation for the interpreter
# Currently we have a C, NASM (linux + other?), and MASM (windows) implementation
set(USE_CPP_INTERP TRUE)
check_language(ASM_NASM)
check_language(ASM_MASM)

if(USE_CPP_INTERP)
    list(APPEND SOURCES "src/vm/vm_c.cpp")
elseif(CMAKE_ASM_NASM_COMPILER)
    enable_language(ASM_NASM)
    list(APPEND SOURCES "src/vm/vm_nasm.asm")
elseif(CMAKE_ASM_MASM_COMPILER)
    enable_language(ASM_MASM)
    list(APPEND SOURCES "src/vm/vm_masm.asm")
else()
    message(FATAL_ERROR "No suitable interpreter implementation found")
endif()

# Define executable sources
add_executable(galevm ${SOURCES})

# Include own headers
target_include_directories(galevm PRIVATE ${VM_SOURCE_DIR}/src)
# Include common headers
target_include_directories(galevm PRIVATE ${Language_SOURCE_DIR}/common)