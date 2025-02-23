//
// Created by Nail Sharipov on 22.11.2024.
//

#ifndef BOOSTAPP_UTIL_H
#define BOOSTAPP_UTIL_H
#include <boost/polygon/polygon.hpp>
#include <boost/polygon/detail/boolean_op.hpp>

using namespace boost::polygon;
using namespace boost::polygon::operators;

typedef polygon_data<int> Polygon;
typedef polygon_set_data<int> PolygonSet;
typedef polygon_traits<Polygon>::point_type Point;

typedef polygon_45_data<int> Polygon45;
typedef polygon_45_set_data<int> PolygonSet45;
typedef polygon_traits<Polygon45>::point_type Point45;

PolygonSet repeat(std::vector<Point> path, int dx, int dy, int n);
PolygonSet manySquares(const Point& start, int size, int offset, int n);
std::pair<PolygonSet, PolygonSet> manyWindows(const Point& start, int a, int b, int offset, int n);
std::pair<PolygonSet, PolygonSet> concentricSquares(int a, int n);
PolygonSet manyLinesX(int a, int n);
PolygonSet manyLinesY(int a, int n);

PolygonSet45 repeat45(const std::vector<Point45>& origin, int dx, int dy, int n);
PolygonSet45 manySquares45(const Point45& start, int size, int offset, int n);
std::pair<PolygonSet45, PolygonSet45> manyWindows45(const Point45& start, int a, int b, int offset, int n);
std::pair<PolygonSet45, PolygonSet45> concentricSquares45(int a, int n);
PolygonSet45 manyLinesX45(int a, int n);
PolygonSet45 manyLinesY45(int a, int n);


//PolygonD spiral(int count, int radius);

#endif //BOOSTAPP_UTIL_H
