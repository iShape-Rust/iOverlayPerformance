//
// Created by Nail Sharipov on 31.01.2024.
//

#include "WindowsTest.h"
#include "Util.h"
#include "include/clipper2/clipper.h"
#include <iostream>
#include <chrono>
#include <cassert>
#include <iomanip>

using namespace Clipper2Lib;

// A grid of square frames, each with a smaller square cutout in the center.
void WindowsTest::run(int n, ClipType clipType) {
    std::cout << "Start Windows Test\n";

    long long offset = 30;
    long long x = n * offset / 2;
    Point64 origin = {-x, -x};

    auto pair = manyWindows(origin, 20, 10, offset, n);

    auto start = std::chrono::high_resolution_clock::now();

    Paths64 solution = BooleanOp(clipType, FillRule::NonZero, pair.first, pair.second);

    auto end = std::chrono::high_resolution_clock::now();
    assert(!solution.empty());

    std::chrono::duration<double> elapsed = end - start;

    int count = 4 * n;
    double count_log = log2(count);

    double time = elapsed.count();
    double time_log = log10(time);

    std::cout << n << "(" << std::fixed << std::setprecision(1) << count_log << ")"
              << "     - " << time << "(" << std::fixed << std::setprecision(1) << time_log << ")\n";
}