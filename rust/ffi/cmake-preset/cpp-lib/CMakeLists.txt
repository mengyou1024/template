cmake_minimum_required(VERSION 3.20)

project(cpp-lib VERSION {{ project_version }} LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD {{cpp_standard}})

add_library(${PROJECT_NAME} STATIC src/add.cpp)

target_include_directories(${PROJECT_NAME} PUBLIC include)

target_link_libraries(${PROJECT_NAME} PRIVATE fmt::fmt-header-only)

if(MSVC)
    target_compile_options(${PROJECT_NAME} PUBLIC "/utf-8" "/W4")
endif(MSVC)

if(WIN32)
    target_compile_definitions(${PROJECT_NAME} PUBLIC UNICODE _UNICODE)
endif(WIN32)

if(BUILD_TESTING)
    include(CTest)
    add_subdirectory(test)
endif()
