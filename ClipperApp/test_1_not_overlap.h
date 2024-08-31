//
// Created by Nail Sharipov on 29.11.2023.
//

#ifndef CLIPPERAPP_TEST_1_NOT_OVERLAP_H
#define CLIPPERAPP_TEST_1_NOT_OVERLAP_H

#include "include/clipper2/clipper.core.h"
#include "include/clipper2/clipper.engine.h"
#include <vector>

using namespace Clipper2Lib;

struct NotOverlapTest {
    static void run(int n, ClipType clipType);
};

// 1
// Union

/*
5     - 0.000005
25     - 0.000021
113     - 0.000097
481     - 0.000457
1985     - 0.002114
8065     - 0.010783
32513     - 0.056281
130561     - 0.369146
523265     - 2.695334
2095105     - 20.665812
8384513     - 167.966801
 */


#endif //CLIPPERAPP_TEST_1_NOT_OVERLAP_H
