use std::time::Instant;
use i_float::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use crate::test::util::Util;

pub(crate) struct CheckerboardTest;

// Xor:
//  3(2.6)       - 0.0001(-4.0)
//  6(4.9)       - 0.0005(-3.3)
//  12(7.0)      - 0.0018(-2.7)
//  25(9.2)      - 0.0087(-2.1)
//  50(11.3)     - 0.0381(-1.4)
//  100(13.3)    - 0.1653(-0.8)
//  200(15.3)    - 0.6729(-0.2)
//  400(17.3)    - 2.8136(0.4)
//  800(19.3)    - 11.3557(1.1)
// 1600(21.3)    - 46.8921(1.7)

// A grid of overlapping squares forming a simple checkerboard pattern.
impl CheckerboardTest {
    pub(crate) fn run(n: usize, rule: OverlayRule) {
        println!("Start Checkerboard Test {:?}", rule);
        let subj_paths = Util::many_squares(IntPoint::new(0, 0), 20, 30, n);
        let clip_paths = Util::many_squares(IntPoint::new(15, 15), 20, 30, n - 1);

        let start = Instant::now();

        let overlay = Overlay::with_paths(&subj_paths, &clip_paths);
        let graph = overlay.build_graph(FillRule::NonZero);
        let result = graph.extract_shapes(rule);

        let duration = start.elapsed();

        assert!(!result.is_empty());

        let count = n * (n - 1);
        let count_log = (count as f64).log2();

        let time = duration.as_secs_f64();
        let time_log = time.log10();

        println!("{}({:.1})     - {:.4}({:.1})", n, count_log, time, time_log);
    }
}