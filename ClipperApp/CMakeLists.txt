cmake_minimum_required(VERSION 3.26)
project(ClipperApp)

set(CMAKE_CXX_STANDARD 17)

add_executable(ClipperApp main.cpp
        GenerateAPI.cpp
        GenerateAPI.h
        NoOverlapTest.cpp
        NoOverlapTest.h
        ManySquaresTest.cpp
        ManySquaresTest.h
        Clipper2Lib/src/clipper.engine.cpp
        Clipper2Lib/src/clipper.offset.cpp
        Clipper2Lib/src/clipper.rectclip.cpp
        UnionTest.cpp
        UnionTest.h
)

include_directories(${PROJECT_SOURCE_DIR}/Clipper2Lib)
