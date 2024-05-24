use std::time::Instant;
use i_float::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use crate::test::util::Util;

pub(crate) struct CheckerboardTest;

// 0
// Xor:
//  3(2.6)       - 0.000051(-4.3)
//  6(4.9)       - 0.000448(-3.3)
//  12(7.0)      - 0.001935(-2.7)
//  25(9.2)      - 0.009461(-2.0)
//  50(11.3)     - 0.041657(-1.4)
//  100(13.3)    - 0.169110(-0.8)
//  200(15.3)    - 0.785906(-0.1)
//  400(17.3)    - 3.351570(0.5)
//  800(19.3)    - 13.944975(1.1)
// 1600(21.3)    - 57.875843(1.8)

//  3(3.7)       - 0.000028(-4.6)
//  6(5.9)       - 0.000242(-3.6)
//  12(8.0)      - 0.001394(-2.9)
//  25(9.2)      - 0.008291(-2.1)
//  50(11.3)     - 0.036966(-1.4)
//  100(13.3)    - 0.163221(-0.8)
//  200(16.3)    - 0.687626(-0.2)
//  400(18.3)    - 3.043982(0.5)
//  800(20.3)    - 12.460823(1.1)
// 1600(22.3)    - 53.687547(1.7)

// A grid of overlapping squares forming a simple checkerboard pattern.
impl CheckerboardTest {
    pub(crate) fn run(n: usize, rule: OverlayRule) {
        let begin = Instant::now();

        println!("Start Checkerboard Test {:?} at {:?}", rule, begin);
        let subj_paths = Util::many_squares(IntPoint::new(0, 0), 20, 30, n);
        let clip_paths = Util::many_squares(IntPoint::new(15, 15), 20, 30, n - 1);

        let start = Instant::now();

        let it_count = ((1000.0 / (n as f64)) as usize).max(1);
        let it_count= it_count * it_count;
        for _ in 0..it_count {
            let overlay = Overlay::with_paths(&subj_paths, &clip_paths);
            let graph = overlay.build_graph_with_solver(FillRule::NonZero, Solver::AUTO);
            _ = graph.extract_shapes(rule);
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / it_count as f64;

        let count = n * n + (n - 1) * (n - 1);
        let count_log = (count as f64).log2();
        let time_log = time.log10();

        println!("{}({:.1})     - {:.6}({:.1})", n, count_log, time, time_log);
    }
}