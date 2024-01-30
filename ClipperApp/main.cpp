#include <iostream>
#include "NoOverlapTest.h"
#include "ManySquaresTest.h"
#include "UnionTest.h"

int main() {
    std::cout << "Start test" << std::endl;
//    NoOverlapTest::run();
    ManySquaresTest::run();
//    UnionTest::run();
    return 0;
}
