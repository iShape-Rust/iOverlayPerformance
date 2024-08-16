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
2(2.3)     - 0.000007(-5.2)
4(4.6)     - 0.000039(-4.4)
8(6.8)     - 0.000213(-3.7)
16(8.9)     - 0.001082(-3.0)
32(11.0)     - 0.005617(-2.3)
64(13.0)     - 0.025686(-1.6)
128(15.0)     - 0.162024(-0.8)
256(17.0)     - 1.117596(0.0)
512(19.0)     - 8.826941(0.9)
1024(21.0)     - 73.885322(1.9)

*/

struct CheckerboardTest {
    static void run(int n, ClipType clipType);
};


#endif //CLIPPERAPP_TEST_0_CHECKERBOARD_H
