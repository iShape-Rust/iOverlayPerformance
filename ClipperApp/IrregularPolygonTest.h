//
// Created by Nail Sharipov on 31.01.2024.
//

#ifndef CLIPPERAPP_IRREGULARPOLYGONTEST_H
#define CLIPPERAPP_IRREGULARPOLYGONTEST_H

#include "include/clipper2/clipper.core.h"
#include "include/clipper2/clipper.engine.h"
#include <vector>

using namespace Clipper2Lib;

struct IrregularPolygonTest {
    static void run(int n, ClipType clipType);
};

#endif //CLIPPERAPP_IRREGULARPOLYGONTEST_H
