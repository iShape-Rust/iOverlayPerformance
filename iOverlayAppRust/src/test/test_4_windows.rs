use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::int::point::IntPoint;
use crate::test::util::Util;

pub(crate) struct WindowsTest;
/*
// 4
// Difference:

// multithreading on
8     - 0.000005
32     - 0.000018
128     - 0.000084
512     - 0.000454
2048     - 0.002018
8192     - 0.009374
32768     - 0.046781
131072     - 0.202866
524288     - 0.873599
2097152     - 3.558283
8388608     - 14.128829

// multithreading off
8     - 0.000005
32     - 0.000017
128     - 0.000079
512     - 0.000436
2048     - 0.002052
8192     - 0.009477
32768     - 0.045476
131072     - 0.212041
524288     - 0.885619
2097152     - 3.645652
8388608     - 15.393479

// fragments

8     - 0.000005
32     - 0.000020
128     - 0.000094
512     - 0.000508
2048     - 0.001657
8192     - 0.007535
32768     - 0.038888
131072     - 0.190919
524288     - 0.817665
2097152     - 3.731909
8388608     - 14.968132
*/

// A grid of square frames, each with a smaller square cutout in the center.
impl WindowsTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver, scale: f64) { // 500
        let offset = 30;
        let x = (n as i32) * offset / 2;
        let origin = IntPoint::new(-x, -x);
        let (subj_paths, clip_paths) = Util::many_windows(origin, 20, 10, offset, n);

        let it_count = ((scale / (n as f64)) as usize).max(1);
        let sq_it_count= it_count * it_count;

        let start = Instant::now();

        for _ in 0..sq_it_count {
            let _ = Overlay::with_contours(&subj_paths, &clip_paths)
                .overlay_with_min_area_and_solver(rule, FillRule::NonZero, 0, solver);
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = 2 * n * n;

        println!("{}     - {:.6}", polygons_count, time);
    }
}