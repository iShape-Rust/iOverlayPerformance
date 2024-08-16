//
// Created by Nail Sharipov on 29.11.2023.
//

#ifndef CLIPPERAPP_TEST_0_CHECKERBOARD_H
#define CLIPPERAPP_TEST_0_CHECKERBOARD_H

#include "include/clipper2/clipper.core.h"
#include "include/clipper2/clipper.engine.h"

using namespace Clipper2Lib;

// Xor:
/*
2(0.7)     - 0.000007(-5.1)
4(1.4)     - 0.000039(-4.4)
8(2.1)     - 0.000213(-3.7)
16(2.7)     - 0.001065(-3.0)
32(3.3)     - 0.005470(-2.3)
64(3.9)     - 0.025387(-1.6)
128(4.5)     - 0.161618(-0.8)
256(5.1)     - 1.122976(0.1)
512(5.7)     - 8.844691(0.9)
1024(6.3)     - 73.136543(1.9)

*/

struct CheckerboardTest {
    static void run(int n, ClipType clipType);
};


#endif //CLIPPERAPP_TEST_0_CHECKERBOARD_H
