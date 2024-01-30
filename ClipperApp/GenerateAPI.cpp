//
// Created by Nail Sharipov on 29.11.2023.
//

#include "GenerateAPI.h"
#include <chrono>
using namespace Clipper2Lib;

Paths64 manySquares(Point64 start, long long size, long long offset, int n) {
    Paths64 subjects;

    auto y = start.x;
    for (int i = 0; i < n; ++i) {
        auto x = start.x;
        for (int j = 0; j < n; ++j) {
            Path64 square = {
                    {x, y},
                    {x, y + size},
                    {x + size, y + size},
                    {x + size, y}
            };
            subjects.push_back(square);
            x += offset;
        }
        y += offset;
    }
    return subjects;
}