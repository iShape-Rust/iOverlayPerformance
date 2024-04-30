use std::time::Instant;
use i_float::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use crate::test::util::Util;

pub(crate) struct CheckerboardTest;

// Xor:
//  3(2.6)       - 0.000063(-4.2)
//  6(4.9)       - 0.000415(-3.4)
//  12(7.0)      - 0.001967(-2.7)
//  24(9.1)      - 0.008770(-2.1)
//  50(11.3)     - 0.042045(-1.4)
//  100(13.3)    - 0.186004(-0.7)
//  200(15.3)    - 0.786722(-0.1)
//  400(17.3)    - 3.420413(0.5)
//  800(19.3)    - 13.966897(1.1)
// 1600(21.3)    - 46.8921(1.7)

// A grid of overlapping squares forming a simple checkerboard pattern.
impl CheckerboardTest {
    pub(crate) fn run(n: usize, rule: OverlayRule) {
        let begin = Instant::now();

        println!("Start Checkerboard Test {:?} at {:?}", rule, begin);
        let subj_paths = Util::many_squares(IntPoint::new(0, 0), 20, 30, n);
        let clip_paths = Util::many_squares(IntPoint::new(15, 15), 20, 30, n - 1);

        let it_count = (400 / n).max(1);
        let mut time = 0.0;
        for _ in 0..it_count {
            let start = Instant::now();

            let overlay = Overlay::with_paths(&subj_paths, &clip_paths);
            let graph = overlay.build_graph_with_solver(FillRule::NonZero, Solver::Auto);
            _ = graph.extract_shapes(rule);

            let duration = start.elapsed();
            time += duration.as_secs_f64();
        }

        time = time / it_count as f64;

        let count = n * (n - 1);
        let count_log = (count as f64).log2();
        let time_log = time.log10();

        println!("{}({:.1})     - {:.6}({:.1})", n, count_log, time, time_log);
    }
}