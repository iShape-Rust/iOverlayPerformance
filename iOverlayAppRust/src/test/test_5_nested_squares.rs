use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use crate::test::util::Util;

pub(crate) struct NestedSquaresTest;

// 5
// Union:

// multithreading on
//  2(32 1.5)               - 0.000011(-5.0)
//  4(128 2.1)              - 0.000022(-4.7)
//  8(512 2.7)              - 0.000047(-4.3)
//  16(2048 3.3)            - 0.000108(-4.0)
//  32(8192 3.9)            - 0.000417(-3.4)
//  64(32768 4.5)           - 0.000673(-3.2)
//  128(131072 5.1)         - 0.001402(-2.9)
//  256(524288 5.7)         - 0.003226(-2.5)
//  512(2097152 6.3)        - 0.006958(-2.2)
//  1024(8388608 6.9)       - 0.014404(-1.8)
//  2048(33554432 7.5)      - 0.034157(-1.5)
//  4096(134217728 8.1)     - 0.069210(-1.2)
//  8192(536870912 8.7)     - 0.174494(-0.8)
// 16384(2147483648 9.3)    - 0.365060(-0.4)
// 32768(8589934592 9.9)    - 1.036588(0.0)
// 65536(34359738368 10.5)  - 2.233662(0.3)
// 131072(137438953472 11.1)- 6.632199(0.8)

// multithreading off
//  2(32 1.5)               - 0.000011(-5.0)
//  4(128 2.1)              - 0.000022(-4.7)
//  8(512 2.7)              - 0.000047(-4.3)
//  16(2048 3.3)            - 0.000108(-4.0)
//  32(8192 3.9)            - 0.000417(-3.4)
//  64(32768 4.5)           - 0.000673(-3.2)
//  128(131072 5.1)         - 0.001402(-2.9)
//  256(524288 5.7)         - 0.003226(-2.5)
//  512(2097152 6.3)        - 0.006816(-2.2)
//  1024(8388608 6.9)       - 0.015233(-1.8)
//  2048(33554432 7.5)      - 0.036489(-1.4)
//  4096(134217728 8.1)     - 0.075042(-1.1)
//  8192(536870912 8.7)     - 0.187803(-0.7)
// 16384(2147483648 9.3)    - 0.395397(-0.4)
// 32768(8589934592 9.9)    - 1.094470(0.0)
// 65536(34359738368 10.5)  - 2.354184(0.4)
// 131072(137438953472 11.1)- 6.982822(0.8)


// A series of concentric squares, each progressively larger than the last.
impl NestedSquaresTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver) {
        println!("Start NestedSquares Test {:?}", rule);
        let a = 4;
        let (subj_paths, clip_paths) = Util::concentric_squares(a, n);

        let start = Instant::now();

        let it_count = ((500.0 / (n as f64)) as usize).max(1);
        let sq_it_count= it_count * it_count;
        for _ in 0..sq_it_count {
            let overlay = Overlay::with_paths(&subj_paths, &clip_paths);
            let graph = overlay.into_graph_with_solver(FillRule::NonZero, solver);
            _ = graph.extract_shapes(rule);
        }
        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let edges_count = 8 * n;
        let count_log = (edges_count as f64).log10();
        let time_log = time.log10();

        println!("{}({} {:.1})     - {:.6}({:.1})", n, edges_count, count_log, time, time_log);
    }
}