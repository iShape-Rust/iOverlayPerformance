use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::point::IntPoint;
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
32     - 0.000018
128     - 0.000084
512     - 0.000456
2048     - 0.002036
8192     - 0.009476
32768     - 0.046010
131072     - 0.215986
524288     - 0.882740
2097152     - 3.663786
8388608     - 15.126074
*/

// A grid of square frames, each with a smaller square cutout in the center.
impl WindowsTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver) {
        let offset = 30;
        let x = (n as i32) * offset / 2;
        let origin = IntPoint::new(-x, -x);
        let (subj_paths, clip_paths) = Util::many_windows(origin, 20, 10, offset, n);

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

        let polygons_count = 2 * n * n;

        println!("{}     - {:.6}", polygons_count, time);
    }
}