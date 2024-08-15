use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use crate::test::util::Util;

pub(crate) struct LinesNetTest;

// 2
// Intersection:

// multithreading on
//  2(4 0.6)        - 0.000007(-5.1)
//  4(8 0.9)        - 0.000022(-4.7)
//  8(16 1.2)       - 0.000082(-4.1)
//  16(32 1.5)      - 0.000660(-3.2)
//  32(64 1.8)      - 0.002773(-2.6)
//  64(128 2.1)     - 0.012898(-1.9)
//  128(256 2.4)    - 0.032715(-1.5)
//  256(512 2.7)    - 0.159289(-0.8)
//  512(1024 3.0)   - 0.588168(-0.2)
// 1024(2048 3.3)   - 2.756978(0.4)
// 2048(4096 3.6)   - 10.965542(1.0)

// multithreading off
//  2(4 0.6)        - 0.000007(-5.1)
//  4(8 0.9)        - 0.000022(-4.7)
//  8(16 1.2)       - 0.000082(-4.1)
//  16(32 1.5)      - 0.000660(-3.2)
//  32(64 1.8)      - 0.002804(-2.6)
//  64(128 2.1)     - 0.012810(-1.9)
//  128(256 2.4)    - 0.035282(-1.5)
//  256(512 2.7)    - 0.165802(-0.8)
//  512(1024 3.0)   - 0.742190(-0.1)
// 1024(2048 3.3)   - 3.318514(0.5)
// 2048(4096 3.6)   - 12.906061(1.1)

// A grid is formed by the intersection of a set of vertical and horizontal lines.
impl LinesNetTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver) {
        println!("Start LinesNet Test {:?}", rule);
        let subj_paths = Util::many_lines_x(20, n);
        let clip_paths = Util::many_lines_y(20, n);

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

        let polygons_count = 2 * n;
        let count_log = (polygons_count as f64).log10();
        let time_log = time.log10();

        println!("{}({} {:.1})     - {:.6}({:.1})", n, polygons_count, count_log, time, time_log);
    }
}