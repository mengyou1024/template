cmake_minimum_required(VERSION 3.20)

if(MINGW)
    # 设置MinGW编译器的字符集
    add_compile_options("-finput-charset=UTF-8" "-fexec-charset=GBK")
endif(MINGW)

project(cpp-lib VERSION {{ project_version }} LANGUAGES C CXX)

if(MINGW)
    # 设置MinGw静态链接, 防止 libstdc++-6.dll 定位错误导致程序无法运行 
    add_link_options("-static")
endif(MINGW)

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
