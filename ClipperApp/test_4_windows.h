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
2(0.6)     - 0.000008(-5.1)
4(1.2)     - 0.000028(-4.5)
8(1.8)     - 0.000114(-3.9)
16(2.4)     - 0.000512(-3.3)
32(3.0)     - 0.002457(-2.6)
64(3.6)     - 0.012430(-1.9)
128(4.2)     - 0.077831(-1.1)
256(4.8)     - 0.562545(-0.2)
512(5.4)     - 4.211770(0.6)
1024(6.0)     - 33.313589(1.5)

 */


using namespace Clipper2Lib;

struct WindowsTest {
    static void run(int n, ClipType clipType);
};
#endif //CLIPPERAPP_TEST_4_WINDOWS_H
