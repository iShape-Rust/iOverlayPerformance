//
// Created by Nail Sharipov on 22.11.2024.
//

#include "test_2_lines_net.h"
#include <vector>
#include <iostream>
#include <iomanip>

void LinesNetTest::run(int n, bool simple_geometry) {
    int it_count = std::max(50 / n, 1);
    int sq_it_count = it_count * it_count;

    std::chrono::duration<double> elapsed{};
    if (simple_geometry) {
        PolygonSet45 subj = manyLinesX45(20, n);
        PolygonSet45 clip = manyLinesY45(20, n);

        auto start = std::chrono::high_resolution_clock::now();

        for (int i = 0; i < sq_it_count; ++i) {
            PolygonSet45 result = subj & clip;
            std::vector<Polygon45> vec_result;
            result.get(vec_result);
        }

        auto end = std::chrono::high_resolution_clock::now();
        elapsed = end - start;
    } else {
        PolygonSet subj = manyLinesX(20, n);
        PolygonSet clip = manyLinesY(20, n);

        auto start = std::chrono::high_resolution_clock::now();

        for (int i = 0; i < sq_it_count; ++i) {
            PolygonSet result = subj & clip;
            std::vector<Polygon> vec_result;
            result.get(vec_result);
        }

        auto end = std::chrono::high_resolution_clock::now();
        elapsed = end - start;
    }

    double time = elapsed.count() / static_cast<double>(sq_it_count);

    int count = 2 * n;

    std::cout << count << "     - " << std::fixed << std::setprecision(6) << time << "\n";
}