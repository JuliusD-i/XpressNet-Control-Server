cmake_minimum_required(VERSION 3.25)
project(xpressnet_rewrite)

enable_testing()

# Add Google Test
find_package(GTest REQUIRED)
include_directories(${GTEST_INCLUDE_DIRS})

set(CMAKE_CXX_STANDARD 20)

# Main executable
add_executable(xpressnet_rewrite
        main.cpp
        lib/xpressnet-lib/xpressnet.cpp
        lib/xpressnet-lib/xpressnet.h
        lib/xpressnet-lib/messages.h
        lib/xpressnet-lib/messages.cpp
)

# Test executable
add_executable(lz_message_definition_filter_test
        tests/lz_message_definition_filter_test.cpp
        lib/xpressnet-lib/messages.cpp
)

# Link the Google Test libraries
target_link_libraries(lz_message_definition_filter_test ${GTEST_LIBRARIES} gtest_main pthread)

add_test(NAME lz_message_definition_filter_test COMMAND lz_message_definition_filter_test)