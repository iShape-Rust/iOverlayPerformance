//
// Created by Nail Sharipov on 31.01.2024.
//

#ifndef CLIPPERAPP_TEST_4_WINDOWS_H
#define CLIPPERAPP_TEST_4_WINDOWS_H

#include "include/clipper2/clipper.core.h"
#include "include/clipper2/clipper.engine.h"
#include <vector>

// 4
// Difference:

/*
8     - 0.000008
32     - 0.000027
128     - 0.000112
512     - 0.000507
2048     - 0.002446
8192     - 0.012282
32768     - 0.076473
131072     - 0.568316
524288     - 4.142673
2097152     - 33.165570
8388608     - 265.387333
 */


using namespace Clipper2Lib;

struct WindowsTest {
    static void run(int n, ClipType clipType);
};
#endif //CLIPPERAPP_TEST_4_WINDOWS_H
