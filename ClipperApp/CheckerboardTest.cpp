//
// Created by Nail Sharipov on 29.11.2023.
//

#include "CheckerboardTest.h"
#include "Util.h"
#include "include/clipper2/clipper.h"
#include <iostream>
#include <chrono>
#include <iomanip>

using namespace Clipper2Lib;

// A grid of overlapping squares forming a simple checkerboard pattern.
void CheckerboardTest::run(int n, ClipType clipType) {
    std::cout << "Start Checkerboard Test\n";

    auto subj = manySquares({0, 0}, 20, 30, n);
    auto clip = manySquares({15, 15}, 20, 30, n - 1);

    int it_count = 1000 / n;
    it_count = it_count < 0 ? 1 : it_count;
    auto start = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < it_count; ++i) {
        Paths64 solution = BooleanOp(clipType, FillRule::NonZero, subj, clip);
    }

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;

    double time = elapsed.count() / (double)it_count;

    int count = n * n + (n - 1) * (n - 1);
    double count_log = log2(count);

    double time_log = log10(time);

    std::cout << n << "(" << std::fixed << std::setprecision(1) << count_log << ")"
              << "     - " <<std::fixed << std::setprecision(6)<< time << "(" << std::fixed << std::setprecision(1) << time_log << ")\n";
}
