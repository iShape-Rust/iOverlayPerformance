//
// Created by Nail Sharipov on 29.11.2023.
//

#ifndef CLIPPERAPP_TEST_1_NOT_OVERLAP_H
#define CLIPPERAPP_TEST_1_NOT_OVERLAP_H

#include "include/clipper2/clipper.core.h"
#include "include/clipper2/clipper.engine.h"
#include <vector>

using namespace Clipper2Lib;

struct NotOverlapTest {
    static void run(int n, ClipType clipType);
};

// 1
// Xor

/*
2(0.7)     - 0.000005(-5.3)
4(1.4)     - 0.000021(-4.7)
8(2.1)     - 0.000101(-4.0)
16(2.7)     - 0.000469(-3.3)
32(3.3)     - 0.002170(-2.7)
64(3.9)     - 0.011366(-1.9)
128(4.5)     - 0.058218(-1.2)
256(5.1)     - 0.375479(-0.4)
512(5.7)     - 2.761692(0.4)
1024(6.3)     - 20.769775(1.3)
 */


#endif //CLIPPERAPP_TEST_1_NOT_OVERLAP_H
