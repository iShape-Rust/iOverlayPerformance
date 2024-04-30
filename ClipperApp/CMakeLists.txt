cmake_minimum_required(VERSION 3.26)
project(ClipperApp)

set(CMAKE_CXX_STANDARD 17)

add_executable(ClipperApp main.cpp
        Util.cpp
        Util.h
        NotOverlapTest.cpp
        NotOverlapTest.h
        CheckerboardTest.cpp
        CheckerboardTest.h
        Clipper2Lib/src/clipper.engine.cpp
        Clipper2Lib/src/clipper.offset.cpp
        Clipper2Lib/src/clipper.rectclip.cpp
        WindowsTest.cpp
        WindowsTest.h
        LinesNetTest.cpp
        LinesNetTest.h
        IrregularPolygonTest.cpp
        IrregularPolygonTest.h
        NestedSquaresTest.cpp
        NestedSquaresTest.h
)

include_directories(${PROJECT_SOURCE_DIR}/Clipper2Lib)
