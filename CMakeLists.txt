cmake_minimum_required(VERSION 3.25)
project(XpressNet-Control-Server)

enable_testing()

# Add Google Test
find_package(GTest REQUIRED)
include_directories(${GTEST_INCLUDE_DIRS})

set(CMAKE_CXX_STANDARD 20)

# Main executable
add_executable(XpressNet-Control-Server
        main.cpp
        lib/xpressnet-lib/xpressnet.cpp
        lib/xpressnet-lib/xpressnet.h
        lib/xpressnet-lib/messages.h
        lib/xpressnet-lib/messages.cpp
)

# Test executable
add_executable(XpressNet-Control-Server-test
        tests/lz_message_definition_filter_test.cpp
        lib/xpressnet-lib/messages.cpp
)

# Link the Google Test libraries
target_link_libraries(XpressNet-Control-Server-test ${GTEST_LIBRARIES} gtest_main pthread)

add_test(NAME XpressNet-Control-Server-test COMMAND lz_message_definition_filter_test)