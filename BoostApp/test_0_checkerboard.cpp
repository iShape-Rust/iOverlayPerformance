//
// Created by Nail Sharipov on 22.11.2024.
//

#include "test_0_checkerboard.h"
#include <vector>
#include <iostream>

namespace bg = boost::geometry;

void CheckerboardTest::run(int n, bool simple_geometry) {
    auto subj = manySquares(Point64(0, 0), 20, 30, n);
    auto clip = manySquares(Point64(15, 15), 20, 30, n - 1);

    // boost is slow, that why it 100 here
    int it_count = std::max(100 / n, 1);
    int sq_it_count = it_count * it_count;
    auto start = std::chrono::high_resolution_clock::now();

    if (simple_geometry) {
        for (int i = 0; i < sq_it_count; ++i) {
            MultiPolygon64 result;
            bg::sym_difference(subj, clip, result, IntersectionStrategy());
        }
    } else {
        for (int i = 0; i < sq_it_count; ++i) {
            MultiPolygon64 result;
            bg::sym_difference(subj, clip, result);
        }
    }

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;
    double time = elapsed.count() / static_cast<double>(sq_it_count);

    int count = n * n + (n - 1) * (n - 1);

    std::cout << count << "     - " << std::fixed << std::setprecision(6) << time << "\n";
}