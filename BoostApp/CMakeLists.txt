cmake_minimum_required(VERSION 3.26)
project(BoostApp)

set(CMAKE_CXX_STANDARD 17)

# Find Boost
find_package(Boost 1.86 REQUIRED)

# Include directories and link Boost if found
if (Boost_FOUND)
    include_directories(${Boost_INCLUDE_DIRS})
    add_executable(BoostApp main.cpp
            Util.cpp
            Util.h

            test_0_checkerboard.cpp
            test_0_checkerboard.h

            test_1_not_overlap.cpp
            test_1_not_overlap.h

            test_2_lines_net.cpp
            test_2_lines_net.h

            test_3_spiral.cpp
            test_3_spiral.h

            test_4_windows.cpp
            test_4_windows.h

            test_5_nested_squares.cpp
            test_5_nested_squares.h
    )

    target_link_libraries(BoostApp ${Boost_LIBRARIES})
else ()
    message(FATAL_ERROR "Boost not found")
endif ()