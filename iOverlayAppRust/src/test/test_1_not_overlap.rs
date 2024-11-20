use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::int::point::IntPoint;
use crate::test::util::Util;

pub(crate) struct NotOverlapTest;
/*

// 1
// Union:

multithreading on

5     - 0.000003
25     - 0.000011
113     - 0.000056
481     - 0.000315
1985     - 0.001569
8065     - 0.008170
32513     - 0.038043
130561     - 0.162694
523265     - 0.734016
2095105     - 3.138123
8384513     - 13.290805

multithreading off

5     - 0.000003
25     - 0.000011
113     - 0.000054
481     - 0.000299
1985     - 0.001556
8065     - 0.008297
32513     - 0.038179
130561     - 0.164280
523265     - 0.751069
2095105     - 3.221546
8384513     - 14.279949

// fragments

5     - 0.000003
25     - 0.000012
113     - 0.000060
481     - 0.000327
1985     - 0.001576
8065     - 0.005867
32513     - 0.028128
130561     - 0.126129
523265     - 0.625514
2095105     - 2.626167
8384513     - 12.642988
*/

// A grid of not overlapping squares.
impl NotOverlapTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver, scale: f64) { // 1000
        let subj_paths = Util::many_squares(IntPoint::new(0, 0), 10, 30, n);
        let clip_paths = Util::many_squares(IntPoint::new(15, 15), 10, 30, n - 1);

        let it_count = ((scale / (n as f64)) as usize).max(1);
        let sq_it_count= it_count * it_count;

        let start = Instant::now();

        for _i in 0..sq_it_count {
            let _ = Overlay::with_contours(&subj_paths, &clip_paths)
                .overlay_with_min_area_and_solver(rule, FillRule::NonZero, 0, solver);
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = n * n + (n - 1) * (n - 1);

        println!("{:.1}     - {:.6}", polygons_count, time);
    }
}