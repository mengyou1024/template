cmake_minimum_required(VERSION 3.20)

project({{ project-name }} VERSION {{ project_version }} LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 23)

{% if build_type == "bin" -%}
add_executable(${PROJECT_NAME} src/main.cpp)
{% elsif build_type == "lib" -%}
add_library(${PROJECT_NAME} {{ library_type }} src/add.cpp)
{% endif %}
target_include_directories(${PROJECT_NAME} PUBLIC include)
{% if build_type == "lib" and library_type == "SHARED" -%}
set_target_properties(${PROJECT_NAME} PROPERTIES RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
{% endif %}
if(MSVC)
    target_compile_options(${PROJECT_NAME} PUBLIC "/source-charset:utf-8" "/W4")
    {%- if build_type == "lib" and library_type == "SHARED" %}
    target_compile_definitions(${PROJECT_NAME} PRIVATE {{ project_name_upper }}_EXPORTS)
    {%- endif %}
endif(MSVC)

if(WIN32)
    target_compile_definitions(${PROJECT_NAME} PUBLIC UNICODE _UNICODE)
endif(WIN32)
{% if build_type == "bin" %}
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

include(GNUInstallDirs)
include(InstallRequiredSystemLibraries)

set(CPACK_PACKAGE_NAME ${PROJECT_NAME})
set(CPACK_PACKAGE_VERSION ${CMAKE_PROJECT_VERSION})
set(CPACK_GENERATOR "ZIP")
set(CPACK_INCLUDE_TOPLEVEL_DIRECTORY OFF)

include(CPack)
{% endif %}
{%- if enable_test %}
if(BUILD_TESTING)
    include(CTest)
    add_subdirectory(test)
endif()
{% endif %}
{%- if enable_rust %}
include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
)
FetchContent_MakeAvailable(Corrosion)

# 导入包或工作空间清单"Cargo.toml"文件
corrosion_import_crate(MANIFEST_PATH rust-src/Cargo.toml)
corrosion_add_cxxbridge(cxxbridge-cpp CRATE rust_src FILES lib.rs foo/mod.rs)
target_include_directories(cxxbridge-cpp PUBLIC "include")
target_link_libraries(${PROJECT_NAME} PUBLIC rust_src cxxbridge-cpp)

if(MSVC)
    # 修复Rust库的运行时
    if(${CMAKE_MSVC_RUNTIME_LIBRARY} MATCHES "^MultiThreaded$")
        corrosion_set_env_vars(rust_src "CFLAGS=-MT" "CXXFLAGS=-MT")
    elseif(${CMAKE_MSVC_RUNTIME_LIBRARY} MATCHES "^MultiThreadedDebug$")
        corrosion_set_env_vars(rust_src "CFLAGS=-MTd" "CXXFLAGS=-MTd")
    elseif(${CMAKE_MSVC_RUNTIME_LIBRARY} MATCHES "^MultiThreadedDLL$")
        corrosion_set_env_vars(rust_src "CFLAGS=-MD" "CXXFLAGS=-MD")
    elseif(${CMAKE_MSVC_RUNTIME_LIBRARY} MATCHES "^MultiThreadedDebugDLL$")
        corrosion_set_env_vars(rust_src "CFLAGS=-MDd" "CXXFLAGS=-MDd")
    endif()
endif()
{% endif -%}