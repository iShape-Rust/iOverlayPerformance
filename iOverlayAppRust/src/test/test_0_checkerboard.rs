use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::point::IntPoint;
use crate::test::util::Util;

pub(crate) struct CheckerboardTest;

/*
// test 0
// Xor:

multithreading on

2(5 0.7)     - 0.000007(-5.2)
4(25 1.4)     - 0.000038(-4.4)
8(113 2.1)     - 0.000204(-3.7)
16(481 2.7)     - 0.001155(-2.9)
32(1985 3.3)     - 0.005351(-2.3)
64(8065 3.9)     - 0.025410(-1.6)
128(32513 4.5)     - 0.108986(-1.0)
256(130561 5.1)     - 0.493565(-0.3)
512(523265 5.7)     - 2.118753(0.3)
1024(2095105 6.3)     - 8.980642(1.0)
2048(8384513 6.9)     - 37.829734(1.6)

multithreading off

2(5 0.7)     - 0.000007(-5.2)
4(25 1.4)     - 0.000038(-4.4)
8(113 2.1)     - 0.000204(-3.7)
16(481 2.7)     - 0.001129(-2.9)
32(1985 3.3)     - 0.005325(-2.3)
64(8065 3.9)     - 0.025059(-1.6)
128(32513 4.5)     - 0.114646(-0.9)
256(130561 5.1)     - 0.519019(-0.3)
512(523265 5.7)     - 2.434005(0.4)
1024(2095105 6.3)     - 9.999720(1.0)
2048(8384513 6.9)     - 42.330557(1.6)
 */

// A grid of overlapping squares forming a simple checkerboard pattern.
impl CheckerboardTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver) {
        let subj_paths = Util::many_squares(IntPoint::new(0, 0), 20, 30, n);
        let clip_paths = Util::many_squares(IntPoint::new(15, 15), 20, 30, n - 1);

        let it_count = ((1000.0 / (n as f64)) as usize).max(1);
        let sq_it_count= it_count * it_count;

        let start = Instant::now();

        for _i in 0..sq_it_count {
            let overlay = Overlay::with_paths(&subj_paths, &clip_paths);
            let graph = overlay.into_graph_with_solver(FillRule::NonZero, solver);
            _ = graph.extract_shapes(rule);
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = n * n + (n - 1) * (n - 1);
        let count_log = (polygons_count as f64).log10();
        let time_log = time.log10();

        println!("{}({} {:.1})     - {:.6}({:.1})", n, polygons_count, count_log, time, time_log);
    }
}