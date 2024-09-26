use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::point::IntPoint;
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
25     - 0.000012
113     - 0.000057
481     - 0.000313
1985     - 0.001581
8065     - 0.008185
32513     - 0.039063
130561     - 0.166031
523265     - 0.776720
2095105     - 3.237469
8384513     - 14.386099

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
            let overlay = Overlay::with_paths(&subj_paths, &clip_paths);
            let graph = overlay.into_graph_with_solver(FillRule::NonZero, solver);
            _ = graph.extract_shapes(rule);
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = n * n + (n - 1) * (n - 1);

        println!("{:.1}     - {:.6}", polygons_count, time);
    }
}