//
// Created by Nail Sharipov on 22.11.2024.
//

#include "test_2_lines_net.h"

void LinesNetTest::run(int n, bool simple_geometry) {
    auto subj = manyLinesX(20, n);
    auto clip = manyLinesY(20, n);

    // boost is slow, that why it 50 here
    int it_count = std::max(50 / n, 1);
    int sq_it_count = it_count * it_count;
    auto start = std::chrono::high_resolution_clock::now();

    if (simple_geometry) {
        for (int i = 0; i < sq_it_count; ++i) {
            MultiPolygon64 result;
            bg::intersection(subj, clip, result, IntersectionStrategy());
        }
    } else {
        for (int i = 0; i < sq_it_count; ++i) {
            MultiPolygon64 result;
            bg::intersection(subj, clip, result);
        }
    }

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;
    double time = elapsed.count() / static_cast<double>(sq_it_count);

    int count = 2 * n;

    std::cout << count << "     - " << std::fixed << std::setprecision(6) << time << "\n";
}