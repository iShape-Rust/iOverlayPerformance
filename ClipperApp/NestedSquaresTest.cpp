//
// Created by Nail Sharipov on 31.01.2024.
//

#include "NestedSquaresTest.h"
#include "Util.h"
#include "include/clipper2/clipper.h"
#include <iostream>
#include <chrono>
#include <cassert>
#include <iomanip>

using namespace Clipper2Lib;

// A series of concentric squares, each progressively larger than the last.
void NestedSquaresTest::run(int n, ClipType clipType) {
    std::cout << "Start NestedSquares Test\n";

    long long a = 4;
    auto tuple = concentricSquares(a, n);
    Paths64 subj = std::get<0>(tuple);
    Paths64 clip = std::get<1>(tuple);

    auto start = std::chrono::high_resolution_clock::now();

    Paths64 solution = BooleanOp(clipType, FillRule::NonZero, subj, clip);

    assert(!solution.empty());

    auto end = std::chrono::high_resolution_clock::now();

    std::chrono::duration<double> elapsed = end - start;

    int count = 4 * n;
    double count_log = log2(count);

    double time = elapsed.count();
    double time_log = log10(time);

    std::cout << n << "(" << std::fixed << std::setprecision(1) << count_log << ")"
              << "     - " << time << "(" << std::fixed << std::setprecision(1) << time_log << ")\n";
}