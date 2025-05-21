use std::hint::black_box;
use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay::ShapeType::{Clip, Subject};
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
25     - 0.000012
113     - 0.000062
481     - 0.000344
1985     - 0.001668
8065     - 0.005425
32513     - 0.024718
130561     - 0.107485
523265     - 0.538060
2095105     - 2.470210
8384513     - 9.601191

multithreading off

5     - 0.000003
25     - 0.000013
113     - 0.000061
481     - 0.000346
1985     - 0.001679
8065     - 0.005912
32513     - 0.028454
130561     - 0.127448
523265     - 0.635898
2095105     - 2.708549
8384513     - 13.514846
*/

// A grid of not overlapping squares.
impl NotOverlapTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver, scale: f64) { // 1000
        let subj_paths = Util::many_squares(IntPoint::new(0, 0), 10, 30, n);
        let clip_paths = Util::many_squares(IntPoint::new(15, 15), 10, 30, n - 1);

        let it_count = ((scale / (n as f64)) as usize).max(1);
        let sq_it_count= it_count * it_count;

        let start = Instant::now();

        let mut overlay = Overlay::new_custom(subj_paths.len() + clip_paths.len(), Default::default(), solver);

        for _i in 0..sq_it_count {
            overlay.clear();
            overlay.add_contours(&subj_paths, Subject);
            overlay.add_contours(&clip_paths, Clip);
            let _ = black_box(overlay.overlay(rule, FillRule::NonZero));
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = n * n + (n - 1) * (n - 1);

        println!("{:.1}     - {:.6}", polygons_count, time);
    }
}