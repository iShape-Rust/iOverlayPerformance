//
// Created by Nail Sharipov on 31.01.2024.
//

#ifndef CLIPPERAPP_TEST_2_LINES_NET_H
#define CLIPPERAPP_TEST_2_LINES_NET_H

#include "include/clipper2/clipper.core.h"
#include "include/clipper2/clipper.engine.h"
#include <vector>

using namespace Clipper2Lib;

// Intersection:
//   12(4.6)     - 0.0(-3.8)
//   25(5.6)     - 0.0(-3.1)
//   50(6.6)     - 0.0(-2.6)
//  100(7.6)     - 0.0(-2.0)
//  200(8.6)     - 0.1(-1.2)
//  400(9.6)     - 0.4(-0.4)
//  800(10.6)    - 2.7(0.4)
//  1600(11.6)     - 21.9(1.3)

struct LinesNetTest {
    static void run(int n, ClipType clipType);
};


#endif //CLIPPERAPP_TEST_2_LINES_NET_H