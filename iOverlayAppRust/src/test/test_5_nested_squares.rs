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
4     - 0.000009
8     - 0.000018
16     - 0.000037
32     - 0.000083
64     - 0.000212
128     - 0.000586
256     - 0.001884
512     - 0.002964
1024     - 0.004924
2048     - 0.010349
4096     - 0.025834
8192     - 0.052011
16384     - 0.133035
32768     - 0.272786
65536     - 0.746876
131072     - 1.561850
262144     - 4.624782
524288     - 10.055448

// multithreading off
4     - 0.000009
8     - 0.000018
16     - 0.000037
32     - 0.000078
64     - 0.000193
128     - 0.000558
256     - 0.001645
512     - 0.002471
1024     - 0.004859
2048     - 0.010348
4096     - 0.025078
8192     - 0.051318
16384     - 0.132727
32768     - 0.273925
65536     - 0.742967
131072     - 1.602064
262144     - 4.685150
524288     - 10.136123
*/

// A series of concentric squares, each progressively larger than the last.
impl NestedSquaresTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver, scale: f64) { // 500
        let (subj_paths, clip_paths) = Util::concentric_squares(4, n);

        let it_count = ((scale / (n as f64)) as usize).max(1);
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