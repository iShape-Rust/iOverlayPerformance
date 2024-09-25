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
8     - 0.000017
16     - 0.000063
32     - 0.000250
64     - 0.001263
128     - 0.005184
256     - 0.026061
512     - 0.133244
1024     - 0.587539
2048     - 2.347003
4096     - 10.460054
 */

// A grid is formed by the intersection of a set of vertical and horizontal lines.
impl LinesNetTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver) {
        let subj_paths = Util::many_lines_x(20, n);
        let clip_paths = Util::many_lines_y(20, n);

        let it_count = ((500.0 / (n as f64)) as usize).max(1);
        let sq_it_count= it_count * it_count;

        let start = Instant::now();

        for _ in 0..sq_it_count {
            let overlay = Overlay::with_paths(&subj_paths, &clip_paths);
            let graph = overlay.into_graph_with_solver(FillRule::NonZero, solver);
            _ = graph.extract_shapes(rule);
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = 2 * n;

        println!("{}     - {:.6}", polygons_count, time);
    }
}