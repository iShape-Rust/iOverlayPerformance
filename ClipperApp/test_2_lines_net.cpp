//
// Created by Nail Sharipov on 31.01.2024.
//

#include "test_2_lines_net.h"
#include "Util.h"
#include "include/clipper2/clipper.h"
#include <iostream>
#include <chrono>
#include <cassert>
#include <iomanip>

using namespace Clipper2Lib;

// A grid is formed by the intersection of a set of vertical and horizontal lines.
void LinesNetTest::run(int n, ClipType clipType) {
    auto subj = manyLinesX(20, n);
    auto clip = manyLinesY(20, n);

    int it_count = std::max(500 / n, 1);
    int sq_it_count = it_count * it_count;
    auto start = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < sq_it_count; ++i) {
        Paths64 solution = BooleanOp(clipType, FillRule::NonZero, subj, clip);
    }

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;
    double time = elapsed.count() / static_cast<double>(sq_it_count);

    int count = 2 * n;
    double count_log = log10(count);
    double time_log = log10(time);

    std::cout << n << "(" << std::fixed << std::setprecision(1) << count_log << ")"
              << "     - " << std::fixed << std::setprecision(6) << time << "("
              << std::fixed << std::setprecision(1) << time_log << ")\n";
}