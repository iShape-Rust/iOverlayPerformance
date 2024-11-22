//
// Created by Nail Sharipov on 22.11.2024.
//

#ifndef BOOSTAPP_UTIL_H
#define BOOSTAPP_UTIL_H
#include <boost/geometry.hpp>
#include <boost/geometry/strategies/cartesian/intersection.hpp>

namespace bg = boost::geometry;

using IntersectionStrategy = bg::strategy::intersection::cartesian_segments<>;

using Point64 = bg::model::d2::point_xy<long long>;
using Polygon64 = bg::model::polygon<Point64>;
using MultiPolygon64 = bg::model::multi_polygon<Polygon64>;

using PointD = bg::model::d2::point_xy<double>;
using PathD = std::vector<PointD>;
using PolygonD = bg::model::polygon<PointD>;
using MultiPolygonD = bg::model::multi_polygon<PolygonD>;

MultiPolygon64 manySquares(Point64 start, long long size, long long offset, int n);
std::pair<MultiPolygon64, MultiPolygon64> manyWindows(Point64 start, long long a, long long b, long long offset, int n);
std::pair<MultiPolygon64, MultiPolygon64> concentricSquares(int a, int n);
PolygonD spiral(int count, int radius);
MultiPolygon64 manyLinesX(long long a, int n);
MultiPolygon64 manyLinesY(long long a, int n);

#endif //BOOSTAPP_UTIL_H
