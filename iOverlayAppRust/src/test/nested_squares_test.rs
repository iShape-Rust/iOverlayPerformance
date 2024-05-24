use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use crate::test::util::Util;

pub(crate) struct NestedSquaresTest;

// 5
// Union:

//   3(3.6)        - 0.000022(-4.7)
//   6(4.6)        - 0.000050(-4.3)
//  12(5.6)        - 0.000227(-3.6)
//  25(6.6)        - 0.000496(-3.3)
//  50(7.6)        - 0.000950(-3.0)
//  100(8.6)       - 0.002022(-2.7)
//  200(9.6)       - 0.004233(-2.4)
//  400(10.6)      - 0.009463(-2.0)
//  800(11.6)      - 0.020562(-1.7)
// 1600(12.6)      - 0.046765(-1.3)
// 3200(13.6)      - 0.105694(-1.0)
// 6400(14.6)      - 0.252145(-0.6)
// 12800(15.6)     - 0.594855(-0.2)
// 25600(16.6)     - 1.740652(0.2)
// 51200(17.6)     - 3.744103(0.6)
// 102400(18.6)    - 10.251690(1.0)
// 204800(19.6)    - 31.720691(1.5)
// 409600(20.6)    - 107.814153(2.0)


// A series of concentric squares, each progressively larger than the last.
impl NestedSquaresTest {
    pub(crate) fn run(n: usize, rule: OverlayRule) {
        println!("Start NestedSquares Test {:?}", rule);
        let a = 4;
        let (subj_paths, clip_paths) = Util::concentric_squares(a, n);

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

        let count = 4 * n;
        let count_log = (count as f64).log2();
        let time_log = time.log10();

        println!("{}({:.1})     - {:.6}({:.1})", n, count_log, time, time_log);
    }
}