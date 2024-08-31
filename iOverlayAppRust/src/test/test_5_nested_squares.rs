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
2(2 0.3)     - 0.000010(-5.0)
4(4 0.6)     - 0.000019(-4.7)
8(8 0.9)     - 0.000039(-4.4)
16(16 1.2)     - 0.000087(-4.1)
32(32 1.5)     - 0.000210(-3.7)
64(64 1.8)     - 0.000507(-3.3)
128(128 2.1)     - 0.001388(-2.9)
256(256 2.4)     - 0.003034(-2.5)
512(512 2.7)     - 0.007753(-2.1)
1024(1024 3.0)     - 0.013416(-1.9)
2048(2048 3.3)     - 0.031126(-1.5)
4096(4096 3.6)     - 0.061821(-1.2)
8192(8192 3.9)     - 0.159341(-0.8)
16384(16384 4.2)     - 0.336900(-0.5)
32768(32768 4.5)     - 0.970132(-0.0)
65536(65536 4.8)     - 2.027074(0.3)
131072(131072 5.1)     - 6.319174(0.8)
262144(262144 5.4)     - 13.315932(1.1)

// multithreading off
2(2 0.3)     - 0.000010(-5.0)
4(4 0.6)     - 0.000019(-4.7)
8(8 0.9)     - 0.000040(-4.4)
16(16 1.2)     - 0.000089(-4.1)
32(32 1.5)     - 0.000208(-3.7)
64(64 1.8)     - 0.000516(-3.3)
128(128 2.1)     - 0.001410(-2.9)
256(256 2.4)     - 0.003003(-2.5)
512(512 2.7)     - 0.008195(-2.1)
1024(1024 3.0)     - 0.012863(-1.9)
2048(2048 3.3)     - 0.032396(-1.5)
4096(4096 3.6)     - 0.066548(-1.2)
8192(8192 3.9)     - 0.175519(-0.8)
16384(16384 4.2)     - 0.359168(-0.4)
32768(32768 4.5)     - 1.008006(0.0)
65536(65536 4.8)     - 2.170099(0.3)
131072(131072 5.1)     - 6.607081(0.8)
262144(262144 5.4)     - 13.874567(1.1)
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

        let polygons_count = n;
        let count_log = (polygons_count as f64).log10();
        let time_log = time.log10();

        println!("{}({} {:.1})     - {:.6}({:.1})", n, polygons_count, count_log, time, time_log);
    }
}