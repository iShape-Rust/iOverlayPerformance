//
// Created by Nail Sharipov on 22.11.2024.
//

#include "test_4_windows.h"
#include <vector>
#include <iostream>
#include <iomanip>

void WindowsTest::run(int n, bool simple_geometry) {
    int offset = 30;
    int x = n * offset / 2;

    // boost is slow, that why it 50 here
    int it_count = std::max(50 / n, 1);
    int sq_it_count = it_count * it_count;

    std::chrono::duration<double> elapsed{};
    if (simple_geometry) {
        Point45 origin = {-x, -x};
        auto pair = manyWindows45(origin, 20, 10, offset, n);
        PolygonSet45 subj = pair.first;
        PolygonSet45 clip = pair.second;

        auto start = std::chrono::high_resolution_clock::now();

        for (int i = 0; i < sq_it_count; ++i) {
            PolygonSet45 result = subj - clip;
            std::vector<Polygon45> vec_result;
            result.get(vec_result);
        }

        auto end = std::chrono::high_resolution_clock::now();
        elapsed = end - start;
    } else {
        Point origin = {-x, -x};
        auto pair = manyWindows(origin, 20, 10, offset, n);
        PolygonSet subj = pair.first;
        PolygonSet clip = pair.second;

        auto start = std::chrono::high_resolution_clock::now();

        for (int i = 0; i < sq_it_count; ++i) {
            PolygonSet result = subj - clip;
            std::vector<Polygon> vec_result;
            result.get(vec_result);
        }

        auto end = std::chrono::high_resolution_clock::now();
        elapsed = end - start;
    }

    double time = elapsed.count() / static_cast<double>(sq_it_count);

    int count = 2 * n * n;

    std::cout << count << "     - " << std::fixed << std::setprecision(6) << time << "\n";
}