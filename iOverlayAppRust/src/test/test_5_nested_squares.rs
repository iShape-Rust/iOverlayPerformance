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
4(4 0.6)     - 0.000020(-4.7)
8(8 0.9)     - 0.000041(-4.4)
16(16 1.2)     - 0.000091(-4.0)
32(32 1.5)     - 0.000223(-3.7)
64(64 1.8)     - 0.000573(-3.2)
128(128 2.1)     - 0.001542(-2.8)
256(256 2.4)     - 0.004830(-2.3)
512(512 2.7)     - 0.006934(-2.2)
1024(1024 3.0)     - 0.013942(-1.9)
2048(2048 3.3)     - 0.032099(-1.5)
4096(4096 3.6)     - 0.065972(-1.2)
8192(8192 3.9)     - 0.168897(-0.8)
16384(16384 4.2)     - 0.356905(-0.4)
32768(32768 4.5)     - 0.998940(-0.0)
65536(65536 4.8)     - 2.146178(0.3)
131072(131072 5.1)     - 6.592194(0.8)
262144(262144 5.4)     - 14.157619(1.2)

// multithreading off
2(2 0.3)     - 0.000010(-5.0)
4(4 0.6)     - 0.000020(-4.7)
8(8 0.9)     - 0.000041(-4.4)
16(16 1.2)     - 0.000092(-4.0)
32(32 1.5)     - 0.000227(-3.6)
64(64 1.8)     - 0.000577(-3.2)
128(128 2.1)     - 0.001594(-2.8)
256(256 2.4)     - 0.004882(-2.3)
512(512 2.7)     - 0.006378(-2.2)
1024(1024 3.0)     - 0.013867(-1.9)
2048(2048 3.3)     - 0.033461(-1.5)
4096(4096 3.6)     - 0.069529(-1.2)
8192(8192 3.9)     - 0.178116(-0.7)
16384(16384 4.2)     - 0.380358(-0.4)
32768(32768 4.5)     - 1.061363(0.0)
65536(65536 4.8)     - 2.272556(0.4)
131072(131072 5.1)     - 6.840823(0.8)
262144(262144 5.4)     - 14.817517(1.2)
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