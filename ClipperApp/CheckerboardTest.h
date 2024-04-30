//
// Created by Nail Sharipov on 29.11.2023.
//

#ifndef CLIPPERAPP_CHECKERBOARDTEST_H
#define CLIPPERAPP_CHECKERBOARDTEST_H

#include "include/clipper2/clipper.core.h"
#include "include/clipper2/clipper.engine.h"

using namespace Clipper2Lib;

// Xor:
//  3(2.6)        - 0.000021(-4.7)
//  6(4.9)        - 0.000122(-3.9)
//  12(7.0)       - 0.000571(-3.2)
//  25(9.2)       - 0.003037(-2.5)
//  50(11.3)      - 0.014218(-1.8)
//  100(13.3)     - 0.082954(-1.1)
//  200(15.3)     - 0.566588(-0.2)
//  400(17.3)     - 4.218559(0.6)
//  800(19.3)     - 34.693970(1.5)
//  1600(21.3)    - 294.7(2.5)

struct CheckerboardTest {
    static void run(int n, ClipType clipType);
};


#endif //CLIPPERAPP_CHECKERBOARDTEST_H
