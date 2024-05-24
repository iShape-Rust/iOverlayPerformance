use std::time::Instant;
use i_float::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use crate::test::util::Util;

pub(crate) struct WindowsTest;

// 4
// Difference:
// 3(2.6)         - 0.0000(-4.6)
// 6(3.6)         - 0.0001(-4.0)
// 12(4.6)        - 0.0004(-3.4)
// 25(5.6)        - 0.0021(-2.7)
// 50(6.6)        - 0.0093(-2.0)
// 100(7.6)       - 0.0447(-1.3)
// 200(8.6)       - 0.2036(-0.7)
// 400(9.6)       - 0.8915(-0.0)
// 800(10.6)      - 3.4765(0.5)
// 1600(11.6)     - 14.6853(1.2)

// A grid of square frames, each with a smaller square cutout in the center.
impl WindowsTest {
    pub(crate) fn run(n: usize, rule: OverlayRule) {
        println!("Start Windows Test {:?}", rule);

        let offset = 30;
        let x = (n as i32) * offset / 2;
        let origin = IntPoint::new(-x, -x);
        let (subj_paths, clip_paths) = Util::many_windows(origin, 20, 10, offset, n);

        let start = Instant::now();
        let it_count = ((500.0 / (n as f64)) as usize).max(1);
        let it_count= it_count * it_count;
        for _ in 0..it_count {
            let overlay = Overlay::with_paths(&subj_paths, &clip_paths);
            let graph = overlay.build_graph(FillRule::NonZero);
            _ = graph.extract_shapes(rule);
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / it_count as f64;

        let count = 2 * n;
        let count_log = (count as f64).log2();
        let time_log = time.log10();

        println!("{}({:.1})     - {:.4}({:.1})", n, count_log, time, time_log);
    }
}