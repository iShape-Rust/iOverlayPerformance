//
// Created by Nail Sharipov on 31.01.2024.
//

#ifndef CLIPPERAPP_TEST_2_LINES_NET_H
#define CLIPPERAPP_TEST_2_LINES_NET_H

#include "include/clipper2/clipper.core.h"
#include "include/clipper2/clipper.engine.h"
#include <vector>

using namespace Clipper2Lib;

// 2
// Intersection:

/*
2(0.6)     - 0.000004(-5.4)
4(0.9)     - 0.000013(-4.9)
8(1.2)     - 0.000043(-4.4)
16(1.5)     - 0.000176(-3.8)
32(1.8)     - 0.000755(-3.1)
64(2.1)     - 0.003465(-2.5)
128(2.4)     - 0.018317(-1.7)
256(2.7)     - 0.115310(-0.9)
512(3.0)     - 0.780700(-0.1)
1024(3.3)     - 5.777785(0.8)
2048(3.6)     - 46.999496(1.7)
 */

struct LinesNetTest {
    static void run(int n, ClipType clipType);
};


#endif //CLIPPERAPP_TEST_2_LINES_NET_H