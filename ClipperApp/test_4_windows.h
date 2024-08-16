//
// Created by Nail Sharipov on 31.01.2024.
//

#ifndef CLIPPERAPP_TEST_4_WINDOWS_H
#define CLIPPERAPP_TEST_4_WINDOWS_H

#include "include/clipper2/clipper.core.h"
#include "include/clipper2/clipper.engine.h"
#include <vector>

using namespace Clipper2Lib;

struct WindowsTest {
    static void run(int n, ClipType clipType);
};

// Difference:
// 3(3.6)        -  0.0(-4.2)
// 6(4.6)        -  0.0(-3.9)
// 12(5.6)        - 0.0(-3.3)
// 25(6.6)        - 0.0(-2.8)
// 50(7.6)        - 0.0(-2.1)
// 100(8.6)       - 0.0(-1.4)
// 200(9.6)       - 0.3(-0.5)
// 400(10.6)      - 2.1(0.3)
// 800(11.6)      - 16.1(1.2)
// 1600(12.6)     - 128.9(2.1)

#endif //CLIPPERAPP_TEST_4_WINDOWS_H
