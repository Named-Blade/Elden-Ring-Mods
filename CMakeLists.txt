cmake_minimum_required(VERSION 3.15)

# Set project name and language
project(ERMods LANGUAGES CXX ASM_MASM)

# Specify the C++ standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Disable iterator debugging (equivalent to /D_ITERATOR_DEBUG_LEVEL=0)
add_definitions(-D_ITERATOR_DEBUG_LEVEL=0)

# Find the required libraries
set(USER32_LIB User32.lib)
set(OLE32_LIB ole32.lib)
set(SHELL32_LIB Shell32.lib)
set(LIBER_LIB ${CMAKE_BINARY_DIR}/libER/Release/libER.lib)
set(LIBER_LIB_S ${CMAKE_BINARY_DIR}/libER/Release/libER_STATIC.lib)
set(ZYCORE_LIB ${CMAKE_BINARY_DIR}/../dependencies/CallHook/thirdparty/zydis/include/Zydis/Zycore/Zycore.lib)
set(ZYDIS_LIB ${CMAKE_BINARY_DIR}/../dependencies/CallHook/thirdparty/zydis/include/Zydis/Zydis.lib)

add_subdirectory(dependencies/libER libER)

# Add include directories
include_directories(${CMAKE_SOURCE_DIR}/dependencies/libER/include)
include_directories(${CMAKE_SOURCE_DIR}/dependencies/libER/symbols)
include_directories(${CMAKE_SOURCE_DIR}/dependencies/CallHook/include)
include_directories(${CMAKE_SOURCE_DIR}/dependencies/Pattern16/include)
include_directories(${CMAKE_SOURCE_DIR}/dependencies/inifile-cpp/include)

#build asm file
function(add_asm_source ASM_SRC ASM_OBJ)
    add_custom_command(
        OUTPUT ${ASM_OBJ}
        COMMAND ml64 /c /nologo /Fo ${ASM_OBJ} ${ASM_SRC}
        DEPENDS ${ASM_SRC}
        COMMENT "Assembling ${ASM_SRC}"
    )
endfunction()

#build mod
function(build_mod MOD_NAME MOD_SOURCE)
set (MOD_ASM ${ARGN})
    if(NOT DEFINED MOD_ASM)
        set(SRC_FILES mods/${MOD_NAME}/${MOD_SOURCE}.cpp)
    else()
        set(ASM_SRC ${CMAKE_SOURCE_DIR}/mods/${MOD_NAME}/${MOD_ASM}.asm)
        set(ASM_OBJ ${CMAKE_SOURCE_DIR}/build/${MOD_ASM}.obj)
        add_asm_source(${ASM_SRC} ${ASM_OBJ})
        set(SRC_FILES mods/${MOD_NAME}/${MOD_SOURCE}.cpp ${ASM_OBJ})
    endif()

    add_library(${MOD_NAME} SHARED ${SRC_FILES})
    add_dependencies(${MOD_NAME} libER)
    add_dependencies(${MOD_NAME} libER_static)
    target_link_libraries(${MOD_NAME} PRIVATE ${LIBER_LIB} ${LIBER_LIB_S} ${ZYCORE_LIB} ${ZYDIS_LIB} ${USER32_LIB} ${OLE32_LIB} ${SHELL32_LIB})
endfunction()

#build Mods
build_mod(uncapper uncapMod uncaplevels)
build_mod(endlessNg endlessNg)
build_mod(damageMod damageMod damageMod)
build_mod(barSize barSize)
build_mod(rallyMod rallyMod)

# Set output directory
set_target_properties(uncapper PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}"
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}"
)
