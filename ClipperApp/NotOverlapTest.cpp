//
// Created by Nail Sharipov on 29.11.2023.
//

#include "NotOverlapTest.h"
#include "Util.h"
#include "include/clipper2/clipper.h"
#include <iostream>
#include <chrono>
#include <iomanip>

using namespace Clipper2Lib;

// A grid of not overlapping squares.
void NotOverlapTest::run(int n) {
    std::cout << "Start NotOverlap Test\n";

    auto subj = manySquares({0, 0}, 20, 30, n);

    int it_count = (int)(1000.0 / (double)n);
    it_count = it_count * it_count;
    it_count = it_count == 0 ? 1 : it_count;
    auto start = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < it_count; ++i) {
        Paths64 solution = Union(subj, FillRule::NonZero);
    }

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;

    double time = elapsed.count() / (double)it_count;

    int count = n * n;
    double count_log = log2(count);

    double time_log = log10(time);

    std::cout << n << "(" << std::fixed << std::setprecision(1) << count_log << ")"
              << "     - " <<std::fixed << std::setprecision(6)<< time << "(" << std::fixed << std::setprecision(1) << time_log << ")\n";
}