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

void run_test_0() {
    std::cout << "run Checkerboard test\n";
    for (int i = 1; i <12; ++i) {
        int n = 1 << i;
        CheckerboardTest::run(n, ClipType::Xor);
    }
}

void run_test_1() {
    std::cout << "run NotOverlap test\n";
    for (int i = 1; i <12; ++i) {
        int n = 1 << i;
        NotOverlapTest::run(n, ClipType::Union);
    }
}

void run_test_2() {
    std::cout << "run LinesNet test\n";
    for (int i = 1; i <12; ++i) {
        int n = 1 << i;
        LinesNetTest::run(n, ClipType::Intersection);
    }
}

void run_test_3() {
    std::cout << "run Spiral test\n";
    for (int i = 1; i <21; ++i) {
        int n = 1 << i;
        SpiralTest::run(n);
    }
}

void run_test_4() {
    std::cout << "run Windows test\n";
    for (int i = 1; i <11; ++i) {
        int n = 1 << i;
        WindowsTest::run(n, ClipType::Difference);
    }
}

void run_test_5() {
    std::cout << "run NestedSquares test\n";
    for (int i = 1; i <13; ++i) {
        int n = 1 << i;
        NestedSquaresTest::run(n, ClipType::Union);
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

    if (argsMap.find("test") == argsMap.end()) {
        std::cerr << "Test number or count is not set\n";
        std::exit(1);
    }

    int test = std::stoi(argsMap["test"]);

    switch (test) {
        case 0:
            run_test_0();
            break;
        case 1:
            run_test_1();
            break;
        case 2:
            run_test_2();
            break;
        case 3:
            run_test_3();
            break;
        case 4:
            run_test_4();
            break;
        case 5:
            run_test_5();
            break;
        default:
            std::cout << "Test is not found\n";
            break;
    }

    return 0;
}