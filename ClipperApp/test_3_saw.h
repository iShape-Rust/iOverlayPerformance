//
// Created by Nail Sharipov on 16.08.2024.
//

#ifndef CLIPPERAPP_TEST_3_SAW_H
#define CLIPPERAPP_TEST_3_SAW_H


#include "include/clipper2/clipper.core.h"
#include "include/clipper2/clipper.engine.h"
#include <vector>

// 3
// Intersection:

/*
2(0.6)     - 0.000004(-5.3)
4(0.9)     - 0.000013(-4.9)
8(1.2)     - 0.000044(-4.4)
16(1.5)     - 0.000178(-3.7)
32(1.8)     - 0.000777(-3.1)
64(2.1)     - 0.003397(-2.5)
128(2.4)     - 0.017417(-1.8)
256(2.7)     - 0.112610(-0.9)
512(3.0)     - 0.738061(-0.1)
1024(3.3)     - 5.559544(0.7)
2048(3.6)     - 43.787482(1.6)

 */
using namespace Clipper2Lib;

struct SawTest {
    static void run(int n, ClipType clipType);
};

#endif //CLIPPERAPP_TEST_3_SAW_H