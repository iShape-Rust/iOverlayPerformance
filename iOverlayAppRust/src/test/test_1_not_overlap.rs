use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use i_overlay::i_float::point::IntPoint;
use crate::test::util::Util;

pub(crate) struct NotOverlapTest;

// 1
// Union:

// multithreading on
//  2(5 0.7)           - 0.000004(-5.4)
//  4(25 1.4)          - 0.000018(-4.7)
//  8(113 2.1)         - 0.000085(-4.1)
//  16(481 2.7)        - 0.000582(-3.2)
//  32(1985 3.3)       - 0.002680(-2.6)
//  64(8065 3.9)       - 0.012375(-1.9)
//  128(32513 4.5)     - 0.052412(-1.3)
//  256(130561 5.1)    - 0.216709(-0.7)
//  512(523265 5.7)    - 1.004525(0.0)
//  1024(2095105 6.3)  - 4.254654(0.6)
//  2048(8384513 6.9)  - 17.610006(1.2)

// multithreading off
//  2(5 0.7)           - 0.000004(-5.4)
//  4(25 1.4)          - 0.000018(-4.7)
//  8(113 2.1)         - 0.000085(-4.1)
//  16(481 2.7)        - 0.000588(-3.2)
//  32(1985 3.3)       - 0.002670(-2.6)
//  64(8065 3.9)       - 0.012287(-1.9)
//  128(32513 4.5)     - 0.055837(-1.3)
//  256(130561 5.1)    - 0.235166(-0.6)
//  512(523265 5.7)    - 1.084478(0.0)
//  1024(2095105 6.3)  - 4.616030(0.7)
//  2048(8384513 6.9)  - 19.415165(1.3)

// A grid of not overlapping squares.
impl NotOverlapTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver) {
        println!("Start NotOverlap Test {:?}", rule);

        let subj_paths = Util::many_squares(IntPoint::new(0, 0), 10, 30, n);
        let clip_paths = Util::many_squares(IntPoint::new(15, 15), 10, 30, n - 1);

        let start = Instant::now();

        let it_count = ((1000.0 / (n as f64)) as usize).max(1);
        let sq_it_count= it_count * it_count;
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