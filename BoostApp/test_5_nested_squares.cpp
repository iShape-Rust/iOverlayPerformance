//
// Created by Nail Sharipov on 22.11.2024.
//

#include "test_5_nested_squares.h"

void NestedSquaresTest::run(int n, bool simple_geometry) {
    auto pair = concentricSquares(4, n);

    // boost is slow, that why it 50 here
    int it_count = std::max(50 / n, 1);
    int sq_it_count = it_count * it_count;
    auto start = std::chrono::high_resolution_clock::now();

    if (simple_geometry) {
        for (int i = 0; i < sq_it_count; ++i) {
            MultiPolygon64 result;
            bg::sym_difference(pair.first, pair.second, result, IntersectionStrategy());
        }
    } else {
        for (int i = 0; i < sq_it_count; ++i) {
            MultiPolygon64 result;
            bg::sym_difference(pair.first, pair.second, result);
        }
    }

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;
    double time = elapsed.count() / static_cast<double>(sq_it_count);

    int count = 2 * n;

    std::cout << count << "     - " << std::fixed << std::setprecision(6) << time << "\n";
}