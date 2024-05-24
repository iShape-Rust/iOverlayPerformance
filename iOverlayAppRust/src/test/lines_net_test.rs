use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use crate::test::util::Util;

pub(crate) struct LinesNetTest;

// 1
// Intersection:
//   12(4.6)       - 0.000546(-3.3)
//   25(5.6)       - 0.003306(-2.5)
//   50(6.6)       - 0.014007(-1.9)
//  100(7.6)       - 0.060510(-1.2)
//  200(8.6)       - 0.250494(-0.6)
//  400(9.6)       - 1.069148(0.0)
//  800(10.6)      - 5.230076(0.7)
//  1600(11.6)     - 20.225835(1.3)

// A grid is formed by the intersection of a set of vertical and horizontal lines.
impl LinesNetTest {
    pub(crate) fn run(n: usize, rule: OverlayRule) {
        println!("Start LinesNet Test {:?}", rule);
        let subj_paths = Util::many_lines_x(20, n);
        let clip_paths = Util::many_lines_y(20, n);

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

        println!("{}({:.1})     - {:.6}({:.1})", n, count_log, time, time_log);
    }
}