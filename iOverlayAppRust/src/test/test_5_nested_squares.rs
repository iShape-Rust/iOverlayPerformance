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
32     - 0.000084
64     - 0.000209
128     - 0.000526
256     - 0.001444
512     - 0.003010
1024     - 0.006304
2048     - 0.012454
4096     - 0.031580
8192     - 0.062490
16384     - 0.159240
32768     - 0.338860
65536     - 0.999980
131072     - 2.100775
262144     - 6.715680
524288     - 14.408006

// multithreading off
4     - 0.000010
8     - 0.000019
16     - 0.000039
32     - 0.000084
64     - 0.000208
128     - 0.000525
256     - 0.001431
512     - 0.003261
1024     - 0.006061
2048     - 0.013174
4096     - 0.032371
8192     - 0.065519
16384     - 0.173478
32768     - 0.373045
65536     - 1.052401
131072     - 2.271715
262144     - 6.968428
524288     - 14.814912
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