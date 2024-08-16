//
// Created by Nail Sharipov on 29.11.2023.
//

#ifndef CLIPPERAPP_UTIL_H
#define CLIPPERAPP_UTIL_H

#include "include/clipper2/clipper.core.h"
using namespace Clipper2Lib;

Paths64 manySquares(Clipper2Lib::Point64 start, long long size, long long offset, int n);
Path64 irregularPolygon(double radius, double angle, int n);
std::pair<Paths64, Paths64> manyWindows(Point64 start, long long a, long long b, long long offset, int n);
std::pair<Paths64, Paths64> concentricSquares(int a, int n);
Paths64 manyLinesX(long long a, int n);
Paths64 manyLinesY(long long a, int n);
Paths64 sawLinesX(long long a, int n);
Paths64 sawLinesY(long long a, int n);

#endif //CLIPPERAPP_UTIL_H
