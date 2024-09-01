//
// Created by Nail Sharipov on 31.01.2024.
//

#ifndef CLIPPERAPP_TEST_5_NESTED_SQUARES_H
#define CLIPPERAPP_TEST_5_NESTED_SQUARES_H

#include "include/clipper2/clipper.core.h"
#include "include/clipper2/clipper.engine.h"
#include <vector>

// 5
// Xor:

/*
4     - 0.000012
8     - 0.000023
16     - 0.000050
32     - 0.000118
64     - 0.000291
128     - 0.000806
256     - 0.003415
512     - 0.015989
1024     - 0.081267
2048     - 0.461883
4096     - 2.347209
8192     - 10.612424
16384     - 46.205474
32768     - 251.260857
65536     - 3502.233611
 */


using namespace Clipper2Lib;

struct NestedSquaresTest {
    static void run(int n, ClipType clipType);
};

#endif //CLIPPERAPP_TEST_5_NESTED_SQUARES_H
