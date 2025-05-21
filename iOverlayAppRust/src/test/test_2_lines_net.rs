use std::hint::black_box;
use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay::ShapeType::{Clip, Subject};
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use crate::test::util::Util;

pub(crate) struct LinesNetTest;

/*

// 2
// Intersection:

multithreading on

4     - 0.000004
8     - 0.000014
16     - 0.000050
32     - 0.000196
64     - 0.001016
128     - 0.003970
256     - 0.020870
512     - 0.096745
1024     - 0.397470
2048     - 1.537385
4096     - 7.696920

multithreading off

4     - 0.000004
8     - 0.000014
16     - 0.000049
32     - 0.000195
64     - 0.001013
128     - 0.004046
256     - 0.018815
512     - 0.088601
1024     - 0.417128
2048     - 1.844754
4096     - 7.514548
*/

// A grid is formed by the intersection of a set of vertical and horizontal lines.
impl LinesNetTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver, scale: f64) { // 500
        let subj_paths = Util::many_lines_x(20, n);
        let clip_paths = Util::many_lines_y(20, n);

        let it_count = ((scale / (n as f64)) as usize).max(1);
        let sq_it_count= it_count * it_count;

        let start = Instant::now();

        let mut overlay = Overlay::new_custom(subj_paths.len() + clip_paths.len(), Default::default(), solver);

        for _ in 0..sq_it_count {
            overlay.clear();
            overlay.add_contours(&subj_paths, Subject);
            overlay.add_contours(&clip_paths, Clip);
            let _ = black_box(overlay.overlay(rule, FillRule::NonZero));
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = 2 * n;

        println!("{}     - {:.6}", polygons_count, time);
    }
}