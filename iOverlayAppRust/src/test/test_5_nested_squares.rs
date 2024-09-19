use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use crate::test::util::Util;

pub(crate) struct NestedSquaresTest;

/*

// 5
// Union:

// multithreading on
4     - 0.000010
8     - 0.000019
16     - 0.000037
32     - 0.000083
64     - 0.000209
128     - 0.000591
256     - 0.001905
512     - 0.002795
1024     - 0.005537
2048     - 0.010977
4096     - 0.030289
8192     - 0.059179
16384     - 0.158096
32768     - 0.331498
65536     - 0.968967
131072     - 2.095021
262144     - 6.507479
524288     - 13.684019

// multithreading off
4     - 0.000010
8     - 0.000018
16     - 0.000037
32     - 0.000083
64     - 0.000211
128     - 0.000589
256     - 0.001855
512     - 0.002804
1024     - 0.005819
2048     - 0.011412
4096     - 0.028736
8192     - 0.061248
16384     - 0.166019
32768     - 0.346209
65536     - 1.022073
131072     - 2.158772
262144     - 6.832136
524288     - 14.246658

*/

// A series of concentric squares, each progressively larger than the last.
impl NestedSquaresTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver) {
        let (subj_paths, clip_paths) = Util::concentric_squares(4, n);

        let it_count = ((500.0 / (n as f64)) as usize).max(1);
        let sq_it_count= it_count * it_count;

        let start = Instant::now();

        for _ in 0..sq_it_count {
            let overlay = Overlay::with_paths(&subj_paths, &clip_paths);
            let graph = overlay.into_graph_with_solver(FillRule::EvenOdd, solver);
            _ = graph.extract_shapes(rule);
        }
        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = 2 * n;

        println!("{}     - {:.6}", polygons_count, time);
    }
}