use std::time::Instant;
use i_float::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use crate::test::util::Util;

pub(crate) struct WindowsTest;

// Difference:
// 3(2.6)         - 0.0001(-4.0)
// 6(3.6)         - 0.0002(-3.6)
// 12(4.6)        - 0.0007(-3.2)
// 25(5.6)        - 0.0030(-2.5)
// 50(6.6)        - 0.0117(-1.9)
// 100(7.6)       - 0.0512(-1.3)
// 200(8.6)       - 0.2159(-0.7)
// 400(9.6)       - 0.8885(-0.1)
// 800(10.6)      - 3.6496(0.6)
// 1600(11.6)     - 15.4913(1.2)

// A grid of square frames, each with a smaller square cutout in the center.
impl WindowsTest {
    pub(crate) fn run(n: usize, rule: OverlayRule) {
        println!("Start Windows Test {:?}", rule);

        let offset = 30;
        let x = (n as i32) * offset / 2;
        let origin = IntPoint::new(-x, -x);
        let (subj_paths, clip_paths) = Util::many_windows(origin, 20, 10, offset, n);

        let start = Instant::now();

        let overlay = Overlay::with_paths(&subj_paths, &clip_paths);
        let graph = overlay.build_graph(FillRule::NonZero);
        let result = graph.extract_shapes(rule);

        let duration = start.elapsed();
        assert!(!result.is_empty());

        let count = 2 * n;
        let count_log = (count as f64).log2();

        let time = duration.as_secs_f64();
        let time_log = time.log10();

        println!("{}({:.1})     - {:.4}({:.1})", n, count_log, time, time_log);
    }
}