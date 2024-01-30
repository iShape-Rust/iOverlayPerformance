//
// Created by Nail Sharipov on 29.11.2023.
//

#include "NoOverlapTest.h"
#include "GenerateAPI.h"
#include "include/clipper2/clipper.h"
#include <iostream>
#include <chrono>

using namespace Clipper2Lib;

// 1000 - 9.5 sec

void NoOverlapTest::run() {
    const int n = 1000;
    auto subjects = manySquares({0, 0}, 20, 30, n);

    auto start = std::chrono::high_resolution_clock::now();

    Paths64 solution = Union(subjects, FillRule::NonZero);

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;

    std::cout << "Spend time: " << elapsed.count() << " seconds\n";
}