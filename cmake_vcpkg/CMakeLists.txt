cmake_minimum_required(VERSION 3.20)

project({{ project-name }} C CXX)

set(CMAKE_CXX_STANDARD 23)

include(cmake/Utils.cmake)

{% if build_type == "bin" -%}
add_subdirectory_path("components")

{% endif -%}
resolve_source_tree("src" SRC_FILES INC_FILES INC_DIRS)

{% if build_type == "lib" -%}
add_library(${PROJECT_NAME} {{ library_type }} ${SRC_FILES} ${INC_FILES})
{% endif -%}
{% if build_type == "bin" -%}
add_executable(${PROJECT_NAME} ${SRC_FILES} ${INC_FILES})
{% endif -%}
target_include_directories(${PROJECT_NAME} PUBLIC include)
target_include_directories(${PROJECT_NAME} PRIVATE ${INC_DIRS})
target_compile_options(
    ${PROJECT_NAME}
    PUBLIC
    "$<$<C_COMPILER_ID:MSVC>:/utf-8>"
    "$<$<CXX_COMPILER_ID:MSVC>:/utf-8>"
    "$<$<C_COMPILER_ID:MSVC>:/W4>"
    "$<$<CXX_COMPILER_ID:MSVC>:/W4>"
)

{% if enable_clang_tidy -%}
add_clang_tidy_pre_build(${PROJECT_NAME})

{% endif -%}
{% if build_type == "bin" -%}
install(TARGETS ${PROJECT_NAME})
install(CODE [[

file(
    GET_RUNTIME_DEPENDENCIES
    RESOLVED_DEPENDENCIES_VAR RESOLVED_DEPS
    UNRESOLVED_DEPENDENCIES_VAR UNRESOLVED_DEPS
    LIBRARIES $<TARGET_FILE:{{ project-name }}>
    DIRECTORIES $<TARGET_FILE_DIR:{{ project-name }}>
    PRE_INCLUDE_REGEXES $<TARGET_FILE_DIR:{{ project-name }}>
    PRE_EXCLUDE_REGEXES "system32"
    POST_INCLUDE_REGEXES $<TARGET_FILE_DIR:{{ project-name }}>
    POST_EXCLUDE_REGEXES "system32"
)

foreach(DEP_LIB ${RESOLVED_DEPS})
    file(INSTALL ${DEP_LIB} DESTINATION ${CMAKE_INSTALL_PREFIX}/bin)
endforeach()

]])

{% endif -%}
{% if os-arch == "linux-x86_64" -%}
include(GNUInstallDirs)
{% endif -%}
{% if build_type == "bin" -%}
include(InstallRequiredSystemLibraries)

set(CPACK_PACKAGE_NAME ${PROJECT_NAME})
set(CPACK_PACKAGE_VERSION {{ project_version }})
set(CPACK_GENERATOR "ZIP")
set(CPACK_INCLUDE_TOPLEVEL_DIRECTORY OFF)

include(CPack)

{% endif -%}
{% if enable_test -%}
if(BUILD_TESTING)
    include(CTest)
    add_subdirectory(test)
endif()

{% endif -%}