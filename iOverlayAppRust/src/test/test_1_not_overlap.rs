use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::point::IntPoint;
use crate::test::util::Util;

pub(crate) struct NotOverlapTest;
/*

// 1
// Union:

multithreading on

2(5 0.7)     - 0.000004(-5.4)
4(25 1.4)     - 0.000018(-4.7)
8(113 2.1)     - 0.000090(-4.0)
16(481 2.7)     - 0.000544(-3.3)
32(1985 3.3)     - 0.002490(-2.6)
64(8065 3.9)     - 0.011667(-1.9)
128(32513 4.5)     - 0.050366(-1.3)
256(130561 5.1)     - 0.207041(-0.7)
512(523265 5.7)     - 0.951342(-0.0)
1024(2095105 6.3)     - 4.077253(0.6)
2048(8384513 6.9)     - 17.192466(1.2)


multithreading off

2(5 0.7)     - 0.000004(-5.4)
4(25 1.4)     - 0.000018(-4.7)
8(113 2.1)     - 0.000088(-4.1)
16(481 2.7)     - 0.000562(-3.3)
32(1985 3.3)     - 0.002583(-2.6)
64(8065 3.9)     - 0.011671(-1.9)
128(32513 4.5)     - 0.053138(-1.3)
256(130561 5.1)     - 0.224685(-0.6)
512(523265 5.7)     - 1.033577(0.0)
1024(2095105 6.3)     - 4.453603(0.6)
2048(8384513 6.9)     - 18.921013(1.3)

 */

// A grid of not overlapping squares.
impl NotOverlapTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver) {
        let subj_paths = Util::many_squares(IntPoint::new(0, 0), 10, 30, n);
        let clip_paths = Util::many_squares(IntPoint::new(15, 15), 10, 30, n - 1);

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