//
// Created by Nail Sharipov on 31.01.2024.
//

#ifndef CLIPPERAPP_NESTEDSQUARESTEST_H
#define CLIPPERAPP_NESTEDSQUARESTEST_H

#include "include/clipper2/clipper.core.h"
#include "include/clipper2/clipper.engine.h"
#include <vector>

using namespace Clipper2Lib;

struct NestedSquaresTest {
    static void run(int n, ClipType clipType);
};

// Union
// 3(3.6)          - 0.0(-4.3)
// 6(4.6)          - 0.0(-4.0)
// 12(5.6)         - 0.0(-4.0)
// 25(6.6)         - 0.0(-3.7)
// 50(7.6)         - 0.0(-3.2)
// 100(8.6)        - 0.0(-2.7)
// 200(9.6)        - 0.0(-2.0)
// 400(10.6)       - 0.0(-1.4)
// 800(11.6)       - 0.3(-0.6)
// 1600(12.6)      - 1.4(0.1)
// 3200(13.6)      - 6.3(0.8)
// 6400(14.6)      - 28.1(1.4)
// 12800(15.6)     - 138.4(2.1)

#endif //CLIPPERAPP_NESTEDSQUARESTEST_H
