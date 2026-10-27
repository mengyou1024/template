enable_testing()

include(FetchContent)

FetchContent_Declare(
    gtest QUIET
    URL https://codeload.github.com/google/googletest/tar.gz/refs/tags/v1.14.0
    DOWNLOAD_EXTRACT_TIMESTAMP TRUE
    EXCLUDE_FROM_ALL
)

set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
set(BUILD_GMOCK OFF CACHE BOOL "" FORCE)

FetchContent_MakeAvailable(gtest)

add_executable(
    ${PROJECT_NAME}_tests
    {% if build_type == "bin" -%}
    test_main.cpp
    {%- elsif build_type == "lib" -%}
    test_add.cpp
    {%- endif %}
)

set_target_properties(${PROJECT_NAME}_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

target_link_libraries(
    ${PROJECT_NAME}_tests
    PRIVATE
    gtest_main
    {%- if build_type == "lib" %}
    ${PROJECT_NAME}
    {%- endif %}
)

include(GoogleTest)

gtest_discover_tests(${PROJECT_NAME}_tests)
