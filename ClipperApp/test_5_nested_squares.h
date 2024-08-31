//
// Created by Nail Sharipov on 31.01.2024.
//

#ifndef CLIPPERAPP_TEST_5_NESTED_SQUARES_H
#define CLIPPERAPP_TEST_5_NESTED_SQUARES_H

#include "include/clipper2/clipper.core.h"
#include "include/clipper2/clipper.engine.h"
#include <vector>

// 5
// Union:

/*
2(0.3)     - 0.000008(-5.1)
4(0.6)     - 0.000015(-4.8)
8(0.9)     - 0.000031(-4.5)
16(1.2)     - 0.000075(-4.1)
32(1.5)     - 0.000212(-3.7)
64(1.8)     - 0.000635(-3.2)
128(2.1)     - 0.003067(-2.5)
256(2.4)     - 0.014930(-1.8)
512(2.7)     - 0.075748(-1.1)
1024(3.0)     - 0.446220(-0.4)
2048(3.3)     - 2.272464(0.4)
4096(3.6)     - 10.766789(1.0)
 */


using namespace Clipper2Lib;

struct NestedSquaresTest {
    static void run(int n, ClipType clipType);
};

#endif //CLIPPERAPP_TEST_5_NESTED_SQUARES_H
