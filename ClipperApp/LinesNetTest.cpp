//
// Created by Nail Sharipov on 31.01.2024.
//

#include "LinesNetTest.h"
#include "Util.h"
#include "include/clipper2/clipper.h"
#include <iostream>
#include <chrono>
#include <cassert>
#include <iomanip>

using namespace Clipper2Lib;

// A grid is formed by the intersection of a set of vertical and horizontal lines.
void LinesNetTest::run(int n, ClipType clipType) {
    std::cout << "Start LinesNet Test\n";

    auto subj = manyLinesX(20, n);
    auto clip = manyLinesY(20, n);

    auto start = std::chrono::high_resolution_clock::now();

    Paths64 solution = BooleanOp(clipType, FillRule::NonZero, subj, clip);

    auto end = std::chrono::high_resolution_clock::now();
    assert(!solution.empty());

    std::chrono::duration<double> elapsed = end - start;

    int count = 2 * n;
    double count_log = log2(count);

    double time = elapsed.count();
    double time_log = log10(time);

    std::cout << n << "(" << std::fixed << std::setprecision(1) << count_log << ")"
              << "     - " << time << "(" << std::fixed << std::setprecision(1) << time_log << ")\n";
}