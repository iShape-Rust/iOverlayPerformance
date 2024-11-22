#include "boost/geometry/geometries/point_xy.hpp"
#include "boost/geometry/geometries/polygon.hpp"
#include "boost/geometry/geometries/multi_polygon.hpp"
#include "Util.h"
#include <vector>
#include <cmath>

MultiPolygon64 manySquares(Point64 start, long long size, long long offset, int n) {
    MultiPolygon64 squares;

    auto y = bg::get<1>(start);
    for (int i = 0; i < n; ++i) {
        auto x = bg::get<0>(start);
        for (int j = 0; j < n; ++j) {
            Polygon64 square;
            bg::append(square.outer(), Point64(x, y));
            bg::append(square.outer(), Point64(x, y + size));
            bg::append(square.outer(), Point64(x + size, y + size));
            bg::append(square.outer(), Point64(x + size, y));
            bg::append(square.outer(), Point64(x, y)); // Close the polygon
            bg::correct(square);
            squares.push_back(square);
            x += offset;
        }
        y += offset;
    }
    return squares;
}

std::pair<MultiPolygon64, MultiPolygon64> manyWindows(Point64 start, long long a, long long b, long long offset, int n) {
    MultiPolygon64 boundaries, holes;

    auto y = bg::get<1>(start);
    long long c = (a - b) / 2;
    long long d = b + c;

    for (int i = 0; i < n; ++i) {
        auto x = bg::get<0>(start);
        for (int j = 0; j < n; ++j) {
            Polygon64 boundary;
            bg::append(boundary.outer(), Point64(x, y));
            bg::append(boundary.outer(), Point64(x, y + a));
            bg::append(boundary.outer(), Point64(x + a, y + a));
            bg::append(boundary.outer(), Point64(x + a, y));
            bg::append(boundary.outer(), Point64(x, y)); // Close the polygon
            bg::correct(boundary);
            boundaries.push_back(boundary);

            Polygon64 hole;
            bg::append(hole.outer(), Point64(x + c, y + c));
            bg::append(hole.outer(), Point64(x + c, y + d));
            bg::append(hole.outer(), Point64(x + d, y + d));
            bg::append(hole.outer(), Point64(x + d, y + c));
            bg::append(hole.outer(), Point64(x + c, y + c)); // Close the polygon
            bg::correct(hole);
            holes.push_back(hole);

            x += offset;
        }
        y += offset;
    }

    return {boundaries, holes};
}

std::pair<MultiPolygon64, MultiPolygon64> concentricSquares(int a, int n) {
    MultiPolygon64 vert;
    vert.reserve(2 * n);
    MultiPolygon64 horz;
    horz.reserve(2 * n);

    long s = 2 * a;
    long r = s;

    for (int i = 0; i < n; ++i) {
        Polygon64 hz_top;
        bg::append(hz_top.outer(), Point64(-r, r - a));
        bg::append(hz_top.outer(), Point64(-r, r));
        bg::append(hz_top.outer(), Point64(r, r));
        bg::append(hz_top.outer(), Point64(r, r - a));
        bg::append(hz_top.outer(), Point64(-r, r - a)); // Close the polygon
        bg::correct(hz_top);

        Polygon64 hz_bot;
        bg::append(hz_bot.outer(), Point64(-r, -r));
        bg::append(hz_bot.outer(), Point64(-r, -r + a));
        bg::append(hz_bot.outer(), Point64(r, -r + a));
        bg::append(hz_bot.outer(), Point64(r, -r));
        bg::append(hz_bot.outer(), Point64(-r, -r)); // Close the polygon
        bg::correct(hz_bot);

        horz.push_back(hz_top);
        horz.push_back(hz_bot);

        Polygon64 vt_left;
        bg::append(vt_left.outer(), Point64(-r, -r));
        bg::append(vt_left.outer(), Point64(-r, r));
        bg::append(vt_left.outer(), Point64(-r + a, r));
        bg::append(vt_left.outer(), Point64(-r + a, -r));
        bg::append(vt_left.outer(), Point64(-r, -r)); // Close the polygon
        bg::correct(vt_left);

        Polygon64 vt_right;
        bg::append(vt_right.outer(), Point64(r - a, -r));
        bg::append(vt_right.outer(), Point64(r - a, r));
        bg::append(vt_right.outer(), Point64(r, r));
        bg::append(vt_right.outer(), Point64(r, -r));
        bg::append(vt_right.outer(), Point64(r - a, -r)); // Close the polygon
        bg::correct(vt_right);

        vert.push_back(vt_left);
        vert.push_back(vt_right);
        r += s;
    }

    return {vert, horz};
}

// Function to create a spiral pattern
PolygonD spiral(int count, int radius) {
    PathD a_path;
    PathD b_path;

    a_path.reserve(4 * count + 1);
    b_path.reserve(2 * count);

    double a = 0.0;
    double r = radius;
    double w = 0.1 * radius;

    PointD p0{0.0, 0.0};

    for (int i = 0; i < count; ++i) {
        double sx = cos(a);
        double sy = sin(a);

        double rr = (i % 2 == 0) ? r + 0.2 * radius : r - 0.2 * radius;

        PointD p{rr * sx, rr * sy};
        PointD v{p.x() - p0.x(), p.y() - p0.y()};

        // Normalize vector
        double l = sqrt(v.x() * v.x() + v.y() * v.y());
        PointD n{v.x() / l, v.y() / l};

        PointD t{-w * n.y(), w * n.x()};

        // Append points to a_path
        a_path.emplace_back(p0.x() + t.x(), p0.y() + t.y());
        a_path.emplace_back(p.x() + t.x(), p.y() + t.y());
        b_path.emplace_back(p0.x() - t.x(), p0.y() - t.y());
        b_path.emplace_back(p.x() - t.x(), p.y() - t.y());

        a += radius / r;
        r = radius * (1.0 + a / (2.0 * M_PI));
        p0 = p;
    }

    // Reverse b_path and append to a_path
    std::reverse(b_path.begin(), b_path.end());
    a_path.insert(a_path.end(), b_path.begin(), b_path.end());
    a_path.push_back(a_path[0]); // close path

    // Create PolygonD from the path
    PolygonD polygon;
    for (const auto& point : a_path) {
        bg::append(polygon.outer(), point);
    }
    bg::correct(polygon);

    return polygon;
}

MultiPolygon64 manyLinesX(long long a, int n) {
    MultiPolygon64 lines;
    lines.reserve(n);
    long long w = a / 2;
    long long s = a * n / 2;
    long long x = -s + w / 2;

    for (int i = 0; i < n; ++i) {
        Polygon64 line;
        bg::append(line.outer(), Point64(x, -s));
        bg::append(line.outer(), Point64(x, s));
        bg::append(line.outer(), Point64(x + w, s));
        bg::append(line.outer(), Point64(x + w, -s));
        bg::append(line.outer(), Point64(x, -s)); // Close the polygon
        bg::correct(line);

        lines.push_back(line);
        x += a;
    }

    return lines;
}

MultiPolygon64 manyLinesY(long long a, int n) {
    MultiPolygon64 lines;
    lines.reserve(n);
    long long h = a / 2;
    long long s = a * n / 2;
    long long y = -s + h / 2;

    for (int i = 0; i < n; ++i) {
        Polygon64 line;
        bg::append(line.outer(), Point64(-s, y));
        bg::append(line.outer(), Point64(s, y));
        bg::append(line.outer(), Point64(s, y - h));
        bg::append(line.outer(), Point64(-s, y - h));
        bg::append(line.outer(), Point64(-s, y)); // Close the polygon
        bg::correct(line);

        lines.push_back(line);
        y += a;
    }

    return lines;
}