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
2(2 0.3)     - 0.000011(-5.0)
4(4 0.6)     - 0.000021(-4.7)
8(8 0.9)     - 0.000045(-4.4)
16(16 1.2)     - 0.000110(-4.0)
32(32 1.5)     - 0.000412(-3.4)
64(64 1.8)     - 0.000622(-3.2)
128(128 2.1)     - 0.001334(-2.9)
256(256 2.4)     - 0.002809(-2.6)
512(512 2.7)     - 0.007329(-2.1)
1024(1024 3.0)     - 0.014039(-1.9)
2048(2048 3.3)     - 0.034045(-1.5)
4096(4096 3.6)     - 0.065098(-1.2)
8192(8192 3.9)     - 0.170364(-0.8)
16384(16384 4.2)     - 0.354813(-0.5)
32768(32768 4.5)     - 1.030309(0.0)
65536(65536 4.8)     - 2.195669(0.3)
131072(131072 5.1)     - 6.712019(0.8)
262144(262144 5.4)     - 14.662582(1.2)


// multithreading off
2(2 0.3)     - 0.000012(-4.9)
4(4 0.6)     - 0.000023(-4.6)
8(8 0.9)     - 0.000050(-4.3)
16(16 1.2)     - 0.000121(-3.9)
32(32 1.5)     - 0.000439(-3.4)
64(64 1.8)     - 0.000686(-3.2)
128(128 2.1)     - 0.001458(-2.8)
256(256 2.4)     - 0.003283(-2.5)
512(512 2.7)     - 0.007099(-2.1)
1024(1024 3.0)     - 0.014571(-1.8)
2048(2048 3.3)     - 0.035614(-1.4)
4096(4096 3.6)     - 0.072511(-1.1)
8192(8192 3.9)     - 0.190787(-0.7)
16384(16384 4.2)     - 0.396822(-0.4)
32768(32768 4.5)     - 1.112080(0.0)
65536(65536 4.8)     - 2.396808(0.4)
131072(131072 5.1)     - 7.179817(0.9)
262144(262144 5.4)     - 15.373949(1.2)

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
            let graph = overlay.into_graph_with_solver(FillRule::NonZero, solver);
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