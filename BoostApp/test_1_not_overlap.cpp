//
// Created by Nail Sharipov on 22.11.2024.
//

#include "test_1_not_overlap.h"
#include <vector>
#include <iostream>
#include <iomanip>

void NotOverlapTest::run(int n, bool simple_geometry) {
    int it_count = std::max(100 / n, 1);
    int sq_it_count = it_count * it_count;

    std::chrono::duration<double> elapsed{};
    if (simple_geometry) {
        PolygonSet45 subj = manySquares45(Point(0, 0), 10, 30, n);
        PolygonSet45 clip = manySquares45(Point(15, 15), 10, 30, n - 1);

        auto start = std::chrono::high_resolution_clock::now();

        for (int i = 0; i < sq_it_count; ++i) {
            PolygonSet45 result = subj | clip;
            std::vector<Polygon45> vec_result;
            result.get(vec_result);
        }

        auto end = std::chrono::high_resolution_clock::now();
        elapsed = end - start;
    } else {
        PolygonSet subj = manySquares({0, 0}, 10, 30, n);
        PolygonSet clip = manySquares({15, 15}, 10, 30, n - 1);

        auto start = std::chrono::high_resolution_clock::now();

        for (int i = 0; i < sq_it_count; ++i) {
            PolygonSet result = subj | clip;
            std::vector<Polygon> vec_result;
            result.get(vec_result);
        }

        auto end = std::chrono::high_resolution_clock::now();
        elapsed = end - start;
    }

    double time = elapsed.count() / static_cast<double>(sq_it_count);

    int count = n * n + (n - 1) * (n - 1);

    std::cout << count << "     - " << std::fixed << std::setprecision(6) << time << "\n";
}