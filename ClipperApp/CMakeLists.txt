cmake_minimum_required(VERSION 3.26)
project(ClipperApp)

set(CMAKE_CXX_STANDARD 17)

add_executable(ClipperApp main.cpp
        Clipper2Lib/src/clipper.engine.cpp
        Clipper2Lib/src/clipper.offset.cpp
        Clipper2Lib/src/clipper.rectclip.cpp
        Util.cpp
        Util.h
        test_0_checkerboard.cpp
        test_0_checkerboard.h
        test_1_not_overlap.cpp
        test_1_not_overlap.h
        test_2_lines_net.cpp
        test_2_lines_net.h
        test_3_saw.h
        test_3_saw.cpp
        test_4_windows.cpp
        test_4_windows.h
        test_5_nested_squares.cpp
        test_5_nested_squares.h
)

include_directories(${PROJECT_SOURCE_DIR}/Clipper2Lib)
