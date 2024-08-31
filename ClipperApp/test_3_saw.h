//
// Created by Nail Sharipov on 16.08.2024.
//

#ifndef CLIPPERAPP_TEST_3_SAW_H
#define CLIPPERAPP_TEST_3_SAW_H


#include "include/clipper2/clipper.core.h"
#include "include/clipper2/clipper.engine.h"
#include <vector>

// 3
// Intersection:

/*
2(0.6)     - 0.000004(-5.4)
4(0.9)     - 0.000013(-4.9)
8(1.2)     - 0.000043(-4.4)
16(1.5)     - 0.000174(-3.8)
32(1.8)     - 0.000756(-3.1)
64(2.1)     - 0.003333(-2.5)
128(2.4)     - 0.016832(-1.8)
256(2.7)     - 0.108595(-1.0)
512(3.0)     - 0.713840(-0.1)
1024(3.3)     - 5.145532(0.7)
2048(3.6)     - 42.410125(1.6)
 */
using namespace Clipper2Lib;

struct SawTest {
    static void run(int n);
};

#endif //CLIPPERAPP_TEST_3_SAW_H