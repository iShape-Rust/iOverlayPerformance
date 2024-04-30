//
// Created by Nail Sharipov on 29.11.2023.
//

#include "NotOverlapTest.h"
#include "Util.h"
#include "include/clipper2/clipper.h"
#include <iostream>
#include <chrono>

using namespace Clipper2Lib;

// A grid of not overlapping squares.
void NotOverlapTest::run(int n) {
    std::cout << "Start NotOverlap Test\n";

    auto subj = manySquares({0, 0}, 20, 30, n);

    auto start = std::chrono::high_resolution_clock::now();

    Paths64 solution = Union(subj, FillRule::NonZero);

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;

    std::cout << "Count: " << n << ", time: " << elapsed.count() << "\n";
}