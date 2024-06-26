//
// Created by Nail Sharipov on 29.11.2023.
//

#include "Util.h"
#include "include/clipper2/clipper.h"
#include <iostream>
#include <chrono>

using namespace Clipper2Lib;

Paths64 manySquares(Point64 start, long long size, long long offset, int n) {
    Paths64 subjects;

    auto y = start.x;
    for (int i = 0; i < n; ++i) {
        auto x = start.x;
        for (int j = 0; j < n; ++j) {
            Path64 square = {
                    {x,        y},
                    {x,        y + size},
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

Path64 irregularPolygon(double radius, double angle, int n) {
    Path64 polygon;
    polygon.reserve(n);
    const double da = M_PI * 0.7;
    double a = angle;

    for (int i = 0; i < n; ++i) {
        double x = radius * cos(a);
        double y = radius * sin(a);
        polygon.emplace_back(static_cast<long long>(x), static_cast<long long>(y));
        a += da;
    }

    return polygon;
}

std::pair<Paths64, Paths64> manyWindows(Point64 start, long long a, long long b, long long offset, int n) {
    Paths64 boundaries, holes;
    boundaries.reserve(n * n);
    holes.reserve(n * n);

    auto y = start.y;
    long long c = (a - b) / 2;
    long long d = b + c;

    for (int i = 0; i < n; ++i) {
        auto x = start.x;
        for (int j = 0; j < n; ++j) {
            Path64 boundary = {
                    {x,     y},
                    {x,     y + a},
                    {x + a, y + a},
                    {x + a, y}
            };
            boundaries.push_back(boundary);

            Path64 hole = {
                    {x + c, y + c},
                    {x + c, y + d},
                    {x + d, y + d},
                    {x + d, y + c}
            };
            holes.push_back(hole);

            x += offset;
        }
        y += offset;
    }

    return {boundaries, holes};
}

std::tuple<Paths64, Paths64> concentricSquares(int a, int n) {
    Paths64 vert;
    vert.reserve(2 * n);
    Paths64 horz;
    horz.reserve(2 * n);

    long s = 2 * a;
    long r = s;

    for (int i = 0; i < n; ++i) {
        Path64 hz_top = {
                {-r, r - a},
                {-r, r},
                {r,  r},
                {r,  r - a}
        };

        Path64 hz_bot = {
                {-r, -r},
                {-r, -r + a},
                {r,  -r + a},
                {r,  -r}
        };

        vert.push_back(hz_top);
        vert.push_back(hz_bot);

        Path64 vt_left = {
                {-r,     -r},
                {-r,     r},
                {-r + a, r},
                {-r + a, -r}
        };

        Path64 vt_right = {
                {r - a, -r},
                {r - a, r},
                {r,     r},
                {r,     -r}
        };

        horz.push_back(vt_left);
        horz.push_back(vt_right);
        r += s;
    }

    return std::make_tuple(vert, horz);
}

Paths64 manyLinesX(long long a, int n) {
    Paths64 lines;
    lines.reserve(n);
    long long w = a / 2;
    long long s = a * n / 2;
    long long x = -s + w / 2;

    for (int i = 0; i < n; ++i) {
        Path64 line = {
                {x,     -s},
                {x,     s},
                {x + w, s},
                {x + w, -s}
        };
        lines.push_back(line);
        x += a;
    }

    return lines;
}

Paths64 manyLinesY(long long a, int n) {
    Paths64 lines;
    lines.reserve(n);
    long long h = a / 2;
    long long s = a * n / 2;
    long long y = -s + h / 2;

    for (int i = 0; i < n; ++i) {
        Path64 line = {
                {-s, y},
                {s,  y},
                {s,  y - h},
                {-s, y - h}
        };
        lines.push_back(line);
        y += a;
    }

    return lines;
}



