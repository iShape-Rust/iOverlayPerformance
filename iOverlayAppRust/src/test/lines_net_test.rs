use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use crate::test::util::Util;

pub(crate) struct LinesNetTest;

// Intersection:
//   12(4.6)     - 0.0009(-3.0)
//   25(5.6)     - 0.0045(-2.3)
//   50(6.6)     - 0.0271(-1.6)
//  100(7.6)     - 0.1864(-0.7)
//  200(8.6)     - 0.5616(-0.3)
//  400(9.6)     - 3.5141(0.5)
//  800(10.6)    - 26.9714(1.4)
//  1600(11.6)   - 233.7291(2.4)

// A grid is formed by the intersection of a set of vertical and horizontal lines.
impl LinesNetTest {
    pub(crate) fn run(n: usize, rule: OverlayRule) {
        println!("Start LinesNet Test {:?}", rule);
        let subj_paths = Util::many_lines_x(20, n);
        let clip_paths = Util::many_lines_y(20, n);

        let start = Instant::now();

        let overlay = Overlay::with_paths(&subj_paths, &clip_paths);
        let graph = overlay.build_graph(FillRule::NonZero);
        let result = graph.extract_shapes(rule);

        let duration = start.elapsed();

        debug_assert!(!result.is_empty());

        let count = 2 * n;
        let count_log = (count as f64).log2();

        let time = duration.as_secs_f64();
        let time_log = time.log10();

        println!("{}({:.1})     - {:.4}({:.1})", n, count_log, time, time_log);
    }
}