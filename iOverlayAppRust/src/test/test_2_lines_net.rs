use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use crate::test::util::Util;

pub(crate) struct LinesNetTest;

/*

// 2
// Intersection:

multithreading on

4     - 0.000006
8     - 0.000017
16     - 0.000063
32     - 0.000257
64     - 0.001268
128     - 0.005768
256     - 0.027286
512     - 0.123429
1024     - 0.524896
2048     - 2.325153
4096     - 9.386460

multithreading off

4     - 0.000006
8     - 0.000018
16     - 0.000060
32     - 0.000247
64     - 0.001332
128     - 0.005028
256     - 0.024158
512     - 0.125619
1024     - 0.526200
2048     - 2.304091
4096     - 10.187985

// fragments

4     - 0.000004
8     - 0.000014
16     - 0.000048
32     - 0.000192
64     - 0.001013
128     - 0.003924
256     - 0.018736
512     - 0.086755
1024     - 0.415207
2048     - 1.747713
4096     - 7.459701

 */

// A grid is formed by the intersection of a set of vertical and horizontal lines.
impl LinesNetTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver, scale: f64) { // 500
        let subj_paths = Util::many_lines_x(20, n);
        let clip_paths = Util::many_lines_y(20, n);

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