//
// Created by Nail Sharipov on 31.01.2024.
//

#include "test_5_nested_squares.h"
#include "Util.h"
#include "include/clipper2/clipper.h"
#include <iostream>
#include <chrono>
#include <cassert>
#include <iomanip>

using namespace Clipper2Lib;

// A series of concentric squares, each progressively larger than the last.
void NestedSquaresTest::run(int n, ClipType clipType) {
    auto pair = concentricSquares(4, n);

    int it_count = std::max(500 / n, 1);
    int sq_it_count = it_count * it_count;
    auto start = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < sq_it_count; ++i) {
        Paths64 solution = BooleanOp(clipType, FillRule::EvenOdd, pair.first, pair.second);
    }

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;
    double time = elapsed.count() / static_cast<double>(sq_it_count);

    int count = 2 * n;

    std::cout << count << "     - " << std::fixed << std::setprecision(6) << time << "\n";
}