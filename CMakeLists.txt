cmake_minimum_required(VERSION 3.15)

# Set project name and language
project(Uncapper LANGUAGES CXX ASM_MASM)

# Specify the C++ standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Disable iterator debugging (equivalent to /D_ITERATOR_DEBUG_LEVEL=0)
add_definitions(-D_ITERATOR_DEBUG_LEVEL=0)

# Find the required libraries
set(USER32_LIB User32.lib)
set(OLE32_LIB ole32.lib)
set(SHELL32_LIB Shell32.lib)
set(LIBER_LIB ${CMAKE_BINARY_DIR}/dependencies/libER/Release/libER.lib)

add_subdirectory(dependencies/libER)

# Add include directories
include_directories(${CMAKE_SOURCE_DIR}/dependencies/libER/include)

# Add assembler source file
set(ASM_SRC ${CMAKE_SOURCE_DIR}/uncapper/uncaplevels.asm)
set(ASM_OBJ ${CMAKE_SOURCE_DIR}/build/uncapLevels.obj)

add_custom_command(
    OUTPUT ${ASM_OBJ}
    COMMAND ml64 /c /nologo /Fo ${ASM_OBJ} ${ASM_SRC}
    DEPENDS ${ASM_SRC}
    COMMENT "Assembling ${ASM_SRC}"
)

# Add the main C++ source file
set(SRC_FILES uncapper/uncapMod.cpp ${ASM_OBJ})

# Add the shared library (DLL)
add_library(uncapper SHARED ${SRC_FILES})

# Link required libraries
target_link_libraries(uncapper PRIVATE ${LIBER_LIB} ${USER32_LIB} ${OLE32_LIB} ${SHELL32_LIB})

# Set output directory
set_target_properties(uncapper PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}"
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}"
)
