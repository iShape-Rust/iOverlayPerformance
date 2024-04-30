//
// Created by Nail Sharipov on 31.01.2024.
//

#include "IrregularPolygonTest.h"
#include "Util.h"
#include "include/clipper2/clipper.h"
#include <iostream>
#include <chrono>
#include <cassert>

using namespace Clipper2Lib;

// Two irregular self-intersecting polygons are generated, the vertices of which are defined by a fixed radius and angle.
void IrregularPolygonTest::run(int n, ClipType clipType) {
    std::cout << "Start IrregularPolygon Test\n";

    auto poly0 = irregularPolygon(1000.0, 0.0, n);
    auto poly1 = irregularPolygon(1000.0, 0.5 * PI, n);

    Paths64 subj;
    subj.push_back(poly0);

    Paths64 clip;
    clip.push_back(poly1);

    auto start = std::chrono::high_resolution_clock::now();

    Paths64 solution = BooleanOp(clipType, FillRule::NonZero, subj, clip);

    assert(!solution.empty());

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;

    std::cout << "Count: " << n << ", time: " << elapsed.count() << "\n";
}