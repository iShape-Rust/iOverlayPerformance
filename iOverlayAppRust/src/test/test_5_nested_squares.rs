use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use crate::test::util::Util;

pub(crate) struct CrossTest;

/*

// 5
// Union:

// multithreading on
4     - 0.000009
8     - 0.000017
16     - 0.000034
32     - 0.000081
64     - 0.000217
128     - 0.000608
256     - 0.002016
512     - 0.002641
1024     - 0.005925
2048     - 0.018777
4096     - 0.044756
8192     - 0.165539
16384     - 0.331655
32768     - 1.148905
65536     - 2.197493
131072     - 8.194153
262144     - 15.285741

// multithreading off
4     - 0.000009
8     - 0.000017
16     - 0.000035
32     - 0.000081
64     - 0.000210
128     - 0.000619
256     - 0.001992
512     - 0.002555
1024     - 0.007851
2048     - 0.024523
4096     - 0.060516
8192     - 0.245160
16384     - 0.485605
32768     - 1.814993
65536     - 4.031631
131072     - 15.731705
262144     - 30.809760
*/

// A series of concentric squares, each progressively larger than the last.
impl CrossTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver, scale: f64) { // 500
        let (subj_paths, clip_paths) = Util::concentric_squares(4, n);

        let it_count = ((scale / (n as f64)) as usize).max(1);
        let sq_it_count= it_count * it_count;

        let start = Instant::now();

        for _ in 0..sq_it_count {
            let _ = Overlay::with_contours(&subj_paths, &clip_paths)
                .overlay_with_min_area_and_solver(rule, FillRule::NonZero, 0, solver);
        }
        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = 2 * n;

        println!("{}     - {:.6}", polygons_count, time);
    }
}