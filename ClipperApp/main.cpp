//#include <iostream>
//#include "NotOverlapTest.h"
//#include "CheckerboardTest.h"
//
//int main() {
//    std::cout << "Start test" << std::endl;
////    NotOverlapTest test;
//    CheckerboardTest test;
//    test.run();
//    return 0;
//}

#include <iostream>
#include <string>
#include <map>
#include <cstdlib>
#include "CheckerboardTest.h"
#include "LinesNetTest.h"
#include "NotOverlapTest.h"
#include "IrregularPolygonTest.h"
#include "WindowsTest.h"
#include "NestedSquaresTest.h"

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

    if (argsMap.find("test") == argsMap.end() || argsMap.find("count") == argsMap.end()) {
        std::cerr << "Test number or count is not set\n";
        std::exit(1);
    }

    int test = std::stoi(argsMap["test"]);
    int count = std::stoi(argsMap["count"]);

    switch (test) {
        case 0:
            CheckerboardTest::run(count, ClipType::Xor);
            break;
        case 1:
            LinesNetTest::run(count, ClipType::Intersection);
            break;
        case 2:
            NotOverlapTest::run(count);
            break;
        case 3:
            IrregularPolygonTest::run(count, ClipType::Intersection);
            break;
        case 4:
            WindowsTest::run(count, ClipType::Difference);
            break;
        case 5:
            NestedSquaresTest::run(count, ClipType::Union);
            break;
        default:
            std::cout << "Test is not found\n";
            break;
    }

    return 0;
}
