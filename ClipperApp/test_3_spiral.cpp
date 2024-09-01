//
// Created by Nail Sharipov on 16.08.2024.
//

#include "test_3_spiral.h"
#include "Util.h"
#include "include/clipper2/clipper.h"
#include <iostream>
#include <chrono>
#include <cassert>
#include <iomanip>

using namespace Clipper2Lib;

// A grid is formed by the intersection of a set of vertical and horizontal lines.
void SpiralTest::run(int n) {
    PathD sp = spiral(n, 100);
    PathsD subj;
    subj.push_back(sp);
    PathsD clip;

    int it_count = std::max(1000 / n, 1);
    int sq_it_count = it_count * it_count;
    auto start = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < sq_it_count; ++i) {
        auto solution = BooleanOp(ClipType::Union, FillRule::NonZero, subj, clip, 2);
    }

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;
    double time = elapsed.count() / static_cast<double>(sq_it_count);

    int count = n;

    std::cout << count << "     - " << std::fixed << std::setprecision(6) << time << "\n";
}