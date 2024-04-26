use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use crate::test::util::Util;

pub(crate) struct NestedSquaresTest;

// Union:

// 3(3.6)       - 0.0001(-4.0)
// 6(4.6)       - 0.0002(-3.7)
// 12(5.6)      - 0.0003(-3.5)
// 25(6.6)      - 0.0008(-3.1)
// 50(7.6)      - 0.0018(-2.8)
// 100(8.6)     - 0.0021(-2.7)
// 200(9.6)     - 0.0040(-2.4)
// 400(10.6)    - 0.0087(-2.1)
// 800(11.6)    - 0.0205(-1.7)
// 1600(12.6)   - 0.0412(-1.4)
// 3200(13.6)   - 0.0960(-1.0)
// 6400(14.6)   - 0.2284(-0.6)
// 12800(15.6)  - 0.5387(-0.3)
// 25600(16.6)  - 1.3690(0.1)
// 51200(17.6)  - 3.3879(0.5)
// 102400(18.6) - 9.0652(1.0)
// 204800(19.6) - 27.2890(1.4)

// A series of concentric squares, each progressively larger than the last.
impl NestedSquaresTest {
    pub(crate) fn run(n: usize, rule: OverlayRule) {
        println!("Start NestedSquares Test {:?}", rule);
        let a = 4;
        let (subj_paths, clip_paths) = Util::concentric_squares(a, n);

        let start = Instant::now();

        let overlay = Overlay::with_paths(&subj_paths, &clip_paths);
        let graph = overlay.build_graph(FillRule::NonZero);
        let result = graph.extract_shapes(rule);

        let duration = start.elapsed();
        assert!(!result.is_empty());

        let count = 4 * n;
        let count_log = (count as f64).log2();

        let time = duration.as_secs_f64();
        let time_log = time.log10();

        println!("{}({:.1})     - {:.4}({:.1})", n, count_log, time, time_log);
    }
}