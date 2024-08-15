use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::point::IntPoint;
use crate::test::util::Util;

pub(crate) struct WindowsTest;

// 4
// Difference:

// multithreading on
//  2(16 1.2)     - 0.000007(-5.1)
//  4(32 1.5)     - 0.000027(-4.6)
//  8(64 1.8)     - 0.000118(-3.9)
//  16(128 2.1)   - 0.000702(-3.2)
//  32(256 2.4)   - 0.003086(-2.5)
//  64(512 2.7)   - 0.013631(-1.9)
//  128(1024 3.0) - 0.061846(-1.2)
//  256(2048 3.3) - 0.273772(-0.6)
//  512(4096 3.6) - 1.143896(0.1)
// 1024(8192 3.9) - 4.757975(0.7)
// 2048(16384 4.2)- 19.028062(1.3)

// multithreading off
//  2(16 1.2)     - 0.000007(-5.1)
//  4(32 1.5)     - 0.000027(-4.6)
//  8(64 1.8)     - 0.000118(-3.9)
//  16(128 2.1)   - 0.000702(-3.2)
//  32(256 2.4)   - 0.003101(-2.5)
//  64(512 2.7)   - 0.013722(-1.9)
//  128(1024 3.0) - 0.064660(-1.2)
//  256(2048 3.3) - 0.288983(-0.5)
//  512(4096 3.6) - 1.240711(0.1)
// 1024(8192 3.9) - 5.158805(0.7)
// 2048(16384 4.2)- 20.420834(1.3)

// A grid of square frames, each with a smaller square cutout in the center.
impl WindowsTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver) {
        println!("Start Windows Test {:?}", rule);

        let offset = 30;
        let x = (n as i32) * offset / 2;
        let origin = IntPoint::new(-x, -x);
        let (subj_paths, clip_paths) = Util::many_windows(origin, 20, 10, offset, n);

        let start = Instant::now();
        let it_count = ((500.0 / (n as f64)) as usize).max(1);
        let sq_it_count= it_count * it_count;
        for _ in 0..sq_it_count {
            let overlay = Overlay::with_paths(&subj_paths, &clip_paths);
            let graph = overlay.into_graph_with_solver(FillRule::NonZero, solver);
            _ = graph.extract_shapes(rule);
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let edges_count = 8 * n;
        let count_log = (edges_count as f64).log10();
        let time_log = time.log10();

        println!("{}({} {:.1})     - {:.6}({:.1})", n, edges_count, count_log, time, time_log);
    }
}