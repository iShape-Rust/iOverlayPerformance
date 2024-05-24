//
// Created by Nail Sharipov on 29.11.2023.
//

#ifndef CLIPPERAPP_CHECKERBOARDTEST_H
#define CLIPPERAPP_CHECKERBOARDTEST_H

#include "include/clipper2/clipper.core.h"
#include "include/clipper2/clipper.engine.h"

using namespace Clipper2Lib;

// Xor:
//  3(2.6)        - 0.000022(-4.7)
//  6(4.9)        - 0.000110(-4.0)
//  12(7.0)       - 0.000566(-3.2)
//  25(9.2)       - 0.003070(-2.5)
//  50(11.3)      - 0.014105(-1.9)
//  100(13.3)     - 0.082121(-1.1)
//  200(15.3)     - 0.558307(-0.3)
//  400(17.3)     - 4.145750(0.6)
//  800(19.3)     - 34.317261(1.5)
//  1600(21.3)    - 294.7(2.5)

struct CheckerboardTest {
    static void run(int n, ClipType clipType);
};


#endif //CLIPPERAPP_CHECKERBOARDTEST_H
