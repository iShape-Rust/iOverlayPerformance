use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::float::single::SingleFloatOverlay;
use std::f64::consts::PI;
use std::time::Instant;
use i_overlay::core::solver::Solver;

pub(crate) struct ConcentricTest;

/*

// 7
// Difference:

// multithreading on
1     - 0.000016
2     - 0.000086
4     - 0.000407
8     - 0.001839
16     - 0.008672
32     - 0.028811
64     - 0.090131
128     - 0.368713
256     - 1.456724
512     - 6.657090
1024     - 31.702080

// multithreading off
1     - 0.000016
2     - 0.000082
4     - 0.000387
8     - 0.001811
16     - 0.008894
32     - 0.036036
64     - 0.127312
128     - 0.510567
256     - 2.102224
512     - 10.343500
1024     - 52.008158
*/

// A series of concentric squares, each progressively larger than the last.
impl ConcentricTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver, scale: f64) {
        let (subj_paths, clip_paths) = Self::geometry(100.0, n);

        let it_count = ((scale / (n as f64)) as usize).max(1);
        let sq_it_count = it_count * it_count;

        let start = Instant::now();

        for _ in 0..sq_it_count {
            subj_paths.overlay_with_filter_and_solver(&clip_paths, rule, FillRule::NonZero, Default::default(), solver);
        }
        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        println!("{}     - {:.6}", n, time);
    }

    fn geometry(size: f64, count: usize) -> (Vec<Vec<[f64; 2]>>, Vec<Vec<[f64; 2]>>) {
        let mut clip = Vec::with_capacity(count);
        let mut subj = Vec::with_capacity(count);

        let mut r = size;
        let scale = 0.8 / size;
        let mut angle = 0.0;
        let rr = 0.5 * size;
        for i in 0..count {
            let body = Self::shape([0.0, 0.0], angle, r, i + 3, -1.0);
            let hole = Self::shape([0.0, 0.0], angle, r + size, i + 3, 1.0);
            subj.push(body);
            subj.push(hole);

            let l = 2.0 * PI * r;
            let n = l * scale;
            let clip_count = n as usize;
            let da = 2.0 * PI / n;
            let mut a = angle;
            for j in 0..clip_count {
                let x = a.cos() * r;
                let y = a.sin() * r;

                let shape = Self::shape([x, y], 0.0, rr, j % 5 + 3, 1.0);
                clip.push(shape);
                a += da;
            }

            r += 2.0 * size;
            angle += 0.05;
        }

        (subj, clip)
    }

    fn shape(center: [f64; 2], angle: f64, radius: f64, count: usize, dir: f64) -> Vec<[f64; 2]> {
        let da: f64 = dir * 2.0 * PI / (count as f64);
        let mut points = Vec::with_capacity(count);

        let mut a = angle;

        for _ in 0..count {
            let x = a.cos() * radius + center[0];
            let y = a.sin() * radius + center[1];
            points.push([x, y]);
            a += da;
        }

        points
    }
}
