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
2(2.3)     - 0.000005(-5.3)
4(4.6)     - 0.000021(-4.7)
8(6.8)     - 0.000101(-4.0)
16(8.9)     - 0.000472(-3.3)
32(11.0)     - 0.002282(-2.6)
64(13.0)     - 0.011778(-1.9)
128(15.0)     - 0.058888(-1.2)
256(17.0)     - 0.375961(-0.4)
512(19.0)     - 2.768382(0.4)
1024(21.0)     - 21.030903(1.3)
 */


#endif //CLIPPERAPP_TEST_1_NOT_OVERLAP_H
