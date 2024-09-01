//
// Created by Nail Sharipov on 16.08.2024.
//

#ifndef CLIPPERAPP_TEST_3_SPIRAL_H
#define CLIPPERAPP_TEST_3_SPIRAL_H


#include "include/clipper2/clipper.core.h"
#include "include/clipper2/clipper.engine.h"
#include <vector>

// 3
// Intersection:

/*
2     - 0.000002
4     - 0.000004
8     - 0.000007
16     - 0.000014
32     - 0.000031
64     - 0.000083
128     - 0.000202
256     - 0.000476
512     - 0.001195
1024     - 0.002941
2048     - 0.007578
4096     - 0.020287
8192     - 0.054647
16384     - 0.181050
32768     - 0.606854
65536     - 2.013809
131072     - 6.547658
262144     - 21.171540
524288     - 72.147615
1048576     - 259.866180
 */
using namespace Clipper2Lib;

struct SpiralTest {
    static void run(int n);
};

#endif //CLIPPERAPP_TEST_3_SPIRAL_H