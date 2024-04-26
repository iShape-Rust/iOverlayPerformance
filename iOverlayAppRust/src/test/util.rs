use std::f64::consts::PI;
use i_float::point::IntPoint;
use i_shape::int::path::IntPath;

pub(super) struct Util;

impl Util {
    pub(super) fn many_squares(start: IntPoint, size: i32, offset: i32, n: usize) -> Vec<IntPath> {
        let mut result = Vec::with_capacity(n * n);
        let mut y = start.y;
        for _ in 0..n {
            let mut x = start.x;
            for _ in 0..n {
                let path: IntPath = vec![
                    IntPoint::new(x, y),
                    IntPoint::new(x, y + size),
                    IntPoint::new(x + size, y + size),
                    IntPoint::new(x + size, y),
                ];
                result.push(path);
                x += offset;
            }
            y += offset;
        }

        result
    }

    pub(super) fn irregular_polygon(radius: f64, angle: f64, n: usize) -> IntPath {
        let mut result = Vec::with_capacity(n);
        let da: f64 = PI * 0.7;
        let mut a: f64 = angle;
        let r = 1024.0 * radius;
        for _ in 0..n {
            let sc = a.sin_cos();

            let x = r * sc.1;
            let y = r * sc.0;

            result.push(IntPoint::new(x as i32, y as i32));
            a += da;
        }

        result
    }

    pub(super) fn many_windows(start: IntPoint, a: i32, b: i32, offset: i32, n: usize) -> (Vec<IntPath>, Vec<IntPath>) {
        let mut boundaries = Vec::with_capacity(n * n);
        let mut holes = Vec::with_capacity(n * n);
        let mut y = start.y;
        let c = (a - b) / 2;
        let d = b + c;
        for _ in 0..n {
            let mut x = start.x;
            for _ in 0..n {
                let boundary: IntPath = vec![
                    IntPoint::new(x, y),
                    IntPoint::new(x, y + a),
                    IntPoint::new(x + a, y + a),
                    IntPoint::new(x + a, y),
                ];
                boundaries.push(boundary);

                let hole: IntPath = vec![
                    IntPoint::new(x + c, y + c),
                    IntPoint::new(x + c, y + d),
                    IntPoint::new(x + d, y + d),
                    IntPoint::new(x + d, y + c),
                ];
                holes.push(hole);

                x += offset;
            }
            y += offset;
        }

        (boundaries, holes)
    }

    pub(super) fn concentric_squares(a: i32, n: usize) -> (Vec<IntPath>, Vec<IntPath>) {
        let mut vert = Vec::with_capacity(2 * n);
        let mut horz = Vec::with_capacity(2 * n);
        let s = 2 * a;
        let mut r = s;
        for _ in 0..n {
            let hz_top: IntPath = vec![
                IntPoint::new(-r, r - a),
                IntPoint::new(-r, r),
                IntPoint::new(r, r),
                IntPoint::new(r, r - a),
            ];
            let hz_bot: IntPath = vec![
                IntPoint::new(-r, -r),
                IntPoint::new(-r, -r + a),
                IntPoint::new(r, -r + a),
                IntPoint::new(r, -r),
            ];
            vert.push(hz_top);
            vert.push(hz_bot);

            let vt_left: IntPath = vec![
                IntPoint::new(-r, -r),
                IntPoint::new(-r, r),
                IntPoint::new(-r + a, r),
                IntPoint::new(-r + a, -r),
            ];
            let vt_right: IntPath = vec![
                IntPoint::new(r - a, -r),
                IntPoint::new(r - a, r),
                IntPoint::new(r, r),
                IntPoint::new(r, -r),
            ];
            horz.push(vt_left);
            horz.push(vt_right);

            r += s;
        }

        (vert, horz)
    }

    pub(super) fn many_lines_x(a: i32, n: usize) -> Vec<IntPath> {
        let w = a / 2;
        let s = a * (n as i32) / 2;
        let mut x = -s + w / 2;
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            let path: IntPath = vec![
                IntPoint::new(x, -s),
                IntPoint::new(x, s),
                IntPoint::new(x + w, s),
                IntPoint::new(x + w, -s),
            ];
            result.push(path);
            x += a;
        }

        result
    }

    pub(super) fn many_lines_y(a: i32, n: usize) -> Vec<IntPath> {
        let h = a / 2;
        let s = a * (n as i32) / 2;
        let mut y = -s + h / 2;
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            let path: IntPath = vec![
                IntPoint::new(-s, y),
                IntPoint::new(s, y),
                IntPoint::new(s, y - h),
                IntPoint::new(-s, y - h),
            ];
            result.push(path);
            y += a;
        }

        result
    }
}