include(FetchContent)
enable_testing()
FetchContent_Declare(gtest
    QUIET
    URL https://codeload.github.com/google/googletest/tar.gz/refs/tags/v1.14.0
    DOWNLOAD_EXTRACT_TIMESTAMP TRUE
    EXCLUDE_FROM_ALL
)
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
set(BUILD_GMOCK OFF CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(gtest)

resolve_source_tree("." TEST_SRC_FILES TEST_INC_FILES TEST_INC_DIRS)

if(TARGET unit_tests)
    target_sources(
        unit_tests
        PRIVATE
        ${TEST_SRC_FILES}
        ${TEST_INC_FILES}
    )
else(TARGET unit_tests)
    add_executable(
        unit_tests
        ${TEST_SRC_FILES}
        ${TEST_INC_FILES}
    )
    target_link_libraries(unit_tests PRIVATE gtest_main)
    include(GoogleTest)
    gtest_discover_tests(unit_tests)
    set_target_properties(unit_tests PROPERTIES RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
endif(TARGET unit_tests)
{% if build_type == "lib" -%}
target_link_libraries(
    unit_tests
    PRIVATE
    ${PROJECT_NAME}
)
{% endif-%}
