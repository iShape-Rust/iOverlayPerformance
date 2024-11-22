#include <iostream>
#include <string>
#include <map>
#include <cstdlib>

#include "test_0_checkerboard.h"
#include "test_1_not_overlap.h"
#include "test_2_lines_net.h"
#include "test_3_spiral.h"
#include "test_4_windows.h"
#include "test_5_nested_squares.h"

void run_test_0(bool simple_geometry) {
    std::cout << "run Checkerboard test\n";
    for (int i = 1; i <9; ++i) {
        int n = 1 << i;
        CheckerboardTest::run(n, simple_geometry);
    }
}

void run_test_1(bool simple_geometry) {
    std::cout << "run NotOverlap test\n";
    for (int i = 1; i <9; ++i) {
        int n = 1 << i;
        NotOverlapTest::run(n, simple_geometry);
    }
}

void run_test_2(bool simple_geometry) {
    std::cout << "run LinesNet test\n";
    for (int i = 1; i <9; ++i) {
        int n = 1 << i;
        LinesNetTest::run(n, simple_geometry);
    }
}

void run_test_3() {
    std::cout << "run Spiral test\n";
    for (int i = 1; i <16; ++i) {
        int n = 1 << i;
        SpiralTest::run(n);
    }
}

void run_test_4(bool simple_geometry) {
    std::cout << "run Windows test\n";
    for (int i = 1; i <12; ++i) {
        int n = 1 << i;
        WindowsTest::run(n, simple_geometry);
    }
}

void run_test_5(bool simple_geometry) {
    std::cout << "run NestedSquares test\n";
    for (int i = 1; i <11; ++i) {
        int n = 1 << i;
        NestedSquaresTest::run(n, simple_geometry);
    }
}

int main(int argc, char* argv[]) {
    std::map<std::string, std::string> argsMap;
    for (int i = 1; i < argc; ++i) {
        std::string arg = argv[i];
        if (arg.substr(0, 2) == "--") {
            std::string key = arg.substr(2);
            std::string value = "true"; // Default value for flags
            if (i + 1 < argc) {
                std::string nextArg = argv[i + 1];
                if (nextArg.substr(0, 2) != "--") {
                    value = nextArg;
                    ++i; // Skip next argument since it's a value, not a key
                }
            }
            argsMap[key] = value;
        }
    }

    // Check for required test argument
    if (argsMap.find("test") == argsMap.end()) {
        std::cerr << "Test number or count is not set\n";
        std::exit(1);
    }

    // Get test number
    int test = std::stoi(argsMap["test"]);

    // Default 'geom' to 0 (false) if not specified
    int geom = 0; // Default value
    if (argsMap.find("geom") != argsMap.end()) {
        geom = (argsMap["geom"] == "true") ? 1 : std::stoi(argsMap["geom"]);
    }

    switch (test) {
        case 0:
            run_test_0(geom);
            break;
        case 1:
            run_test_1(geom);
            break;
        case 2:
            run_test_2(geom);
            break;
        case 3:
            run_test_3();
            break;
        case 4:
            run_test_4(geom);
            break;
        case 5:
            run_test_5(geom);
            break;
        default:
            std::cout << "Test is not found\n";
            break;
    }

    return 0;
}
