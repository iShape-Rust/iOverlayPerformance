use std::f64::consts::PI;
use i_float::fix_vec::FixVec;
use i_shape::fix_path::FixPath;

pub(super) struct Util;

impl Util {
    pub(super) fn many_squares(start: FixVec, size: i64, offset: i64, n: usize) -> Vec<FixPath> {
        let mut result = Vec::with_capacity(n * n);
        let mut y = start.y;
        for _ in 0..n {
            let mut x = start.x;
            for _ in 0..n {
                let path: FixPath = vec![
                    FixVec::new(x, y),
                    FixVec::new(x, y + size),
                    FixVec::new(x + size, y + size),
                    FixVec::new(x + size, y),
                ];
                result.push(path);
                x += offset;
            }
            y += offset;
        }

        result
    }

    pub(super) fn random_polygon(radius: f64, n: usize) -> FixPath {
        let mut result = Vec::with_capacity(n);
        let da: f64 = PI * 0.7;
        let mut a: f64 = 0.0;
        for _ in 0..n {
            let sc = a.sin_cos();

            let x = radius * sc.1;
            let y = radius * sc.0;

            result.push(FixVec::new_f64(x, y));
            a += da;
        }

        result
    }

    pub(super) fn many_windows(start: FixVec, a: i64, b: i64, offset: i64, n: usize) -> (Vec<FixPath>, Vec<FixPath>) {
        let mut boundaries = Vec::with_capacity(n * n);
        let mut holes = Vec::with_capacity(n * n);
        let mut y = start.y;
        let c = (a - b) / 2;
        let d = b + c;
        for _ in 0..n {
            let mut x = start.x;
            for _ in 0..n {
                let boundary: FixPath = vec![
                    FixVec::new(x, y),
                    FixVec::new(x, y + a),
                    FixVec::new(x + a, y + a),
                    FixVec::new(x + a, y),
                ];
                boundaries.push(boundary);

                let hole: FixPath = vec![
                    FixVec::new(x + c, y + c),
                    FixVec::new(x + c, y + d),
                    FixVec::new(x + d, y + d),
                    FixVec::new(x + d, y + c),
                ];
                holes.push(hole);

                x += offset;
            }
            y += offset;
        }

        (boundaries, holes)
    }

    pub(super) fn concentric_squares(a: i64, b: i64, n: usize) -> Vec<FixPath> {
        let mut result = Vec::with_capacity(n);
        let mut r = a;
        for _ in 0..n {
            let path: FixPath = vec![
                FixVec::new(-r, -r),
                FixVec::new(-r, r),
                FixVec::new(r, r),
                FixVec::new(r, -r),
            ];
            result.push(path);
            r += b;
        }

        result
    }

    pub(super) fn many_lines_x(a: i64, n: usize) -> Vec<FixPath> {
        let w = a / 2;
        let s = a * (n as i64) / 2;
        let mut x = -s + w / 2;
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            let path: FixPath = vec![
                FixVec::new(x, -s),
                FixVec::new(x , s),
                FixVec::new(x + w, s),
                FixVec::new(x + w, -s)
            ];
            result.push(path);
            x += a;
        }

        result
    }

    pub(super) fn many_lines_y(a: i64, n: usize) -> Vec<FixPath> {
        let h = a / 2;
        let s = a * (n as i64) / 2;
        let mut y = -s + h / 2;
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            let path: FixPath = vec![
                FixVec::new(-s, y),
                FixVec::new(s , y),
                FixVec::new(s, y - h),
                FixVec::new(-s, y - h)
            ];
            result.push(path);
            y += a;
        }

        result
    }

}