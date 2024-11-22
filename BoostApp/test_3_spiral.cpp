//
// Created by Nail Sharipov on 22.11.2024.
//

#include "test_3_spiral.h"

void SpiralTest::run(int n) {
    auto spiral_polygon = spiral(20, n);

    MultiPolygonD subject;
    subject.push_back(spiral_polygon);

    MultiPolygonD clip;

    int it_count = std::max(100 / n, 1);
    int sq_it_count = it_count * it_count;
    auto start = std::chrono::high_resolution_clock::now();

    for (int i = 0; i < sq_it_count; ++i) {
        MultiPolygonD result;
        bg::union_(subject, clip, result);
    }

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;
    double time = elapsed.count() / static_cast<double>(sq_it_count);

    int count = n;

    std::cout << count << "     - " << std::fixed << std::setprecision(6) << time << "\n";
}

