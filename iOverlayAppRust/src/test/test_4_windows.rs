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
2(4 0.6)     - 0.000007(-5.2)
4(16 1.2)     - 0.000028(-4.6)
8(64 1.8)     - 0.000128(-3.9)
16(256 2.4)     - 0.000669(-3.2)
32(1024 3.0)     - 0.002938(-2.5)
64(4096 3.6)     - 0.013314(-1.9)
128(16384 4.2)     - 0.059863(-1.2)
256(65536 4.8)     - 0.262307(-0.6)
512(262144 5.4)     - 1.131158(0.1)
1024(1048576 6.0)     - 4.590453(0.7)
2048(4194304 6.6)     - 18.458689(1.3)

// multithreading off
2(4 0.6)     - 0.000007(-5.2)
4(16 1.2)     - 0.000028(-4.5)
8(64 1.8)     - 0.000129(-3.9)
16(256 2.4)     - 0.000688(-3.2)
32(1024 3.0)     - 0.003050(-2.5)
64(4096 3.6)     - 0.014436(-1.8)
128(16384 4.2)     - 0.066332(-1.2)
256(65536 4.8)     - 0.292598(-0.5)
512(262144 5.4)     - 1.221658(0.1)
1024(1048576 6.0)     - 5.069474(0.7)
2048(4194304 6.6)     - 20.502224(1.3)

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

        let polygons_count = n * n;
        let count_log = (polygons_count as f64).log10();
        let time_log = time.log10();

        println!("{}({} {:.1})     - {:.6}({:.1})", n, polygons_count, count_log, time, time_log);
    }
}