//
// Created by Nail Sharipov on 29.11.2023.
//

#ifndef CLIPPERAPP_NOTOVERLAPTEST_H
#define CLIPPERAPP_NOTOVERLAPTEST_H

#include "include/clipper2/clipper.core.h"
#include <vector>

using namespace Clipper2Lib;

struct NotOverlapTest {
    static void run(int n);
};

// 2
// Union
// 3(3.2)          - 0.000008(-5.1)
// 6(5.2)          - 0.000032(-4.5)
// 12(7.2)         - 0.000133(-3.9)
// 25(9.3)         - 0.000675(-3.2)
// 50(11.3)        - 0.003144(-2.5)
// 100(13.3)       - 0.016100(-1.8)
// 200(15.3)       - 0.097051(-1.0)
// 400(17.3)       - 0.686262(-0.2)
// 800(19.3)       - 5.131059(0.7)
// 1600(21.3)      - 41.576766(1.6)

#endif //CLIPPERAPP_NOTOVERLAPTEST_H
