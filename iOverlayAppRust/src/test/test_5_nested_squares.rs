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
16     - 0.000038
32     - 0.000083
64     - 0.000210
128     - 0.000593
256     - 0.001883
512     - 0.002724
1024     - 0.005558
2048     - 0.010744
4096     - 0.026263
8192     - 0.053851
16384     - 0.137991
32768     - 0.287206
65536     - 0.767102
131072     - 1.640510
262144     - 4.726510
524288     - 10.513923

// multithreading off
4     - 0.000010
8     - 0.000019
16     - 0.000039
32     - 0.000084
64     - 0.000216
128     - 0.000608
256     - 0.001964
512     - 0.002736
1024     - 0.005108
2048     - 0.011339
4096     - 0.026789
8192     - 0.052864
16384     - 0.134364
32768     - 0.292781
65536     - 0.781251
131072     - 1.688798
262144     - 4.809529
524288     - 10.273028
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