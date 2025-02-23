#include "Util.h"
#include <vector>

PolygonSet manySquares(const Point& start, int size, int offset, int n) {
    PolygonSet polygonSet;

    auto y = start.y();
    for (int i = 0; i < n; ++i) {
        auto x = start.x();
        for (int j = 0; j < n; ++j) {
            std::vector<Point> path = {
                    Point(x, y),
                    Point(x + size, y),
                    Point(x + size, y + size),
                    Point(x, y + size)
            };
            Polygon square;
            square.set(path.begin(), path.end());
            polygonSet.insert(square);

            x += offset;
        }
        y += offset;
    }

    return polygonSet;
}

PolygonSet45 manySquares45(const Point45& start, int size, int offset, int n) {
    PolygonSet45 polygonSet;

    auto y = start.y();
    for (int i = 0; i < n; ++i) {
        auto x = start.x();
        for (int j = 0; j < n; ++j) {
            std::vector<Point45> path = {
                    Point45(x, y),
                    Point45(x + size, y),
                    Point45(x + size, y + size),
                    Point45(x, y + size)
            };
            Polygon45 square;
            square.set(path.begin(), path.end());
            polygonSet.insert(square);

            x += offset;
        }
        y += offset;
    }

    return polygonSet;
}

std::pair<PolygonSet, PolygonSet> manyWindows(const Point& start, int a, int b, int offset, int n) {
    PolygonSet boundaries, holes;
    boundaries.reserve(n * n);
    holes.reserve(n * n);

    auto y = start.y();
    int c = (a - b) / 2;
    int d = b + c;

    for (int i = 0; i < n; ++i) {
        auto x = start.x();
        for (int j = 0; j < n; ++j) {
            // Boundary polygon
            std::vector<Point> boundary_outer = {
                    Point(x, y),
                    Point(x + a, y),
                    Point(x + a, y + a),
                    Point(x, y + a)
            };
            Polygon boundary;
            boundary.set(boundary_outer.begin(), boundary_outer.end());
            boundaries.insert(boundary);

            // Hole polygon
            std::vector<Point> hole_outer = {
                    Point(x + c, y + c),
                    Point(x + d, y + c),
                    Point(x + d, y + d),
                    Point(x + c, y + d)
            };
            Polygon hole;
            hole.set(hole_outer.begin(), hole_outer.end());
            holes.insert(hole);

            x += offset;
        }
        y += offset;
    }

    return {boundaries, holes};
}

std::pair<PolygonSet45, PolygonSet45> manyWindows45(const Point45& start, int a, int b, int offset, int n) {
    PolygonSet45 boundaries, holes;
    boundaries.reserve(n * n);
    holes.reserve(n * n);

    auto y = start.y();
    int c = (a - b) / 2;
    int d = b + c;

    for (int i = 0; i < n; ++i) {
        auto x = start.x();
        for (int j = 0; j < n; ++j) {
            // Boundary polygon
            std::vector<Point45> boundary_outer = {
                    Point45(x, y),
                    Point45(x + a, y),
                    Point45(x + a, y + a),
                    Point45(x, y + a)
            };
            Polygon45 boundary;
            boundary.set(boundary_outer.begin(), boundary_outer.end());
            boundaries.insert(boundary);

            // Hole polygon
            std::vector<Point45> hole_outer = {
                    Point45(x + c, y + c),
                    Point45(x + d, y + c),
                    Point45(x + d, y + d),
                    Point45(x + c, y + d)
            };
            Polygon45 hole;
            hole.set(hole_outer.begin(), hole_outer.end());
            holes.insert(hole);

            x += offset;
        }
        y += offset;
    }

    return {boundaries, holes};
}

std::pair<PolygonSet, PolygonSet> concentricSquares(int a, int n) {
    PolygonSet vert;
    vert.reserve(2 * n);
    PolygonSet horz;
    horz.reserve(2 * n);

    int s = 2 * a;
    int r = s;

    for (int i = 0; i < n; ++i) {
        // Horizontal polygons
        std::vector<Point> hz_top_outer = {
                Point(-r, r - a),
                Point(r, r - a),
                Point(r, r),
                Point(-r, r)
        };
        Polygon hz_top;
        hz_top.set(hz_top_outer.begin(), hz_top_outer.end());

        std::vector<Point> hz_bot_outer = {
                Point(-r, -r),
                Point(r, -r),
                Point(r, -r + a),
                Point(-r, -r + a)
        };
        Polygon hz_bot;
        hz_bot.set(hz_bot_outer.begin(), hz_bot_outer.end());

        horz.insert(hz_top);
        horz.insert(hz_bot);

        // Vertical polygons
        std::vector<Point> vt_left_outer = {
                Point(-r, -r),
                Point(-r + a, -r),
                Point(-r + a, r),
                Point(-r, r)
        };
        Polygon vt_left;
        vt_left.set(vt_left_outer.begin(), vt_left_outer.end());

        std::vector<Point> vt_right_outer = {
                Point(r - a, -r),
                Point(r, -r),
                Point(r, r),
                Point(r - a, r)
        };
        Polygon vt_right;
        vt_right.set(vt_right_outer.begin(), vt_right_outer.end());

        vert.insert(vt_left);
        vert.insert(vt_right);

        r += s;
    }

    return {vert, horz};
}

std::pair<PolygonSet45, PolygonSet45> concentricSquares45(int a, int n) {
    PolygonSet45 vert;
    vert.reserve(2 * n);
    PolygonSet45 horz;
    horz.reserve(2 * n);

    int s = 2 * a;
    int r = s;

    for (int i = 0; i < n; ++i) {
        // Horizontal polygons
        std::vector<Point45> hz_top_outer = {
                Point45(-r, r - a),
                Point45(r, r - a),
                Point45(r, r),
                Point45(-r, r)
        };
        Polygon45 hz_top;
        hz_top.set(hz_top_outer.begin(), hz_top_outer.end());

        std::vector<Point45> hz_bot_outer = {
                Point45(-r, -r),
                Point45(r, -r),
                Point45(r, -r + a),
                Point45(-r, -r + a)
        };
        Polygon45 hz_bot;
        hz_bot.set(hz_bot_outer.begin(), hz_bot_outer.end());

        horz.insert(hz_top);
        horz.insert(hz_bot);

        // Vertical polygons
        std::vector<Point45> vt_left_outer = {
                Point45(-r, -r),
                Point45(-r + a, -r),
                Point45(-r + a, r),
                Point45(-r, r)
        };
        Polygon45 vt_left;
        vt_left.set(vt_left_outer.begin(), vt_left_outer.end());

        std::vector<Point45> vt_right_outer = {
                Point45(r - a, -r),
                Point45(r, -r),
                Point45(r, r),
                Point45(r - a, r)
        };
        Polygon45 vt_right;
        vt_right.set(vt_right_outer.begin(), vt_right_outer.end());

        vert.insert(vt_left);
        vert.insert(vt_right);

        r += s;
    }

    return {vert, horz};
}

PolygonSet manyLinesX(int a, int n) {
    PolygonSet lines;
    lines.reserve(n);
    int w = a / 2;
    int s = a * n / 2;
    int x = -s + w / 2;

    for (int i = 0; i < n; ++i) {
        std::vector<Point> line_outer = {
                Point(x, -s),
                Point(x + w, -s),
                Point(x + w, s),
                Point(x, s)
        };
        Polygon line;
        line.set(line_outer.begin(), line_outer.end());
        lines.insert(line);

        x += a;
    }

    return lines;
}

PolygonSet45 manyLinesX45(int a, int n) {
    PolygonSet45 lines;
    lines.reserve(n);
    int w = a / 2;
    int s = a * n / 2;
    int x = -s + w / 2;

    for (int i = 0; i < n; ++i) {
        std::vector<Point45> line_outer = {
                Point45(x, -s),
                Point45(x + w, -s),
                Point45(x + w, s),
                Point45(x, s)
        };
        Polygon45 line;
        line.set(line_outer.begin(), line_outer.end());
        lines.insert(line);

        x += a;
    }

    return lines;
}

PolygonSet manyLinesY(int a, int n) {
    PolygonSet lines;
    lines.reserve(n);
    int h = a / 2;
    int s = a * n / 2;
    int y = -s + h / 2;

    for (int i = 0; i < n; ++i) {
        std::vector<Point> line_outer = {
                Point(-s, y),
                Point(s, y),
                Point(s, y - h),
                Point(-s, y - h)
        };
        Polygon line;
        line.set(line_outer.begin(), line_outer.end());
        lines.insert(line);

        y += a;
    }

    return lines;
}

PolygonSet45 manyLinesY45(int a, int n) {
    PolygonSet45 lines;
    lines.reserve(n);
    int h = a / 2;
    int s = a * n / 2;
    int y = -s + h / 2;

    for (int i = 0; i < n; ++i) {
        std::vector<Point45> line_outer = {
                Point45(-s, y),
                Point45(s, y),
                Point45(s, y - h),
                Point45(-s, y - h)
        };
        Polygon45 line;
        line.set(line_outer.begin(), line_outer.end());
        lines.insert(line);

        y += a;
    }

    return lines;
}

PolygonSet45 repeat45(const std::vector<Point45>& origin, int dx, int dy, int n) {
    PolygonSet45 polygonSet;

    int y = 0;
    for (int i = 0; i < n; ++i) {
        int x = 0;
        for (int j = 0; j < n; ++j) {
            std::vector<Point45> path = origin;
            for (auto& point : path) {
                point.set(HORIZONTAL, point.x() + x);
                point.set(VERTICAL, point.y() + y);
            }

            Polygon45 square;
            square.set(path.begin(), path.end());
            polygonSet.insert(square);

            x += dx;
        }
        y += dy;
    }

    return polygonSet;
}

/*
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
*/