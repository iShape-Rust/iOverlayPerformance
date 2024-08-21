use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use crate::test::util::Util;

pub(crate) struct LinesNetTest;

/*

// 2
// Intersection:

multithreading on

2(4 0.6)     - 0.000006(-5.2)
4(8 0.9)     - 0.000020(-4.7)
8(16 1.2)     - 0.000074(-4.1)
16(32 1.5)     - 0.000297(-3.5)
32(64 1.8)     - 0.001267(-2.9)
64(128 2.1)     - 0.005513(-2.3)
128(256 2.4)     - 0.022986(-1.6)
256(512 2.7)     - 0.106737(-1.0)
512(1024 3.0)     - 0.439352(-0.4)
1024(2048 3.3)     - 2.361429(0.4)
2048(4096 3.6)     - 9.557422(1.0)


multithreading off

2(4 0.6)     - 0.000006(-5.2)
4(8 0.9)     - 0.000020(-4.7)
8(16 1.2)     - 0.000072(-4.1)
16(32 1.5)     - 0.000305(-3.5)
32(64 1.8)     - 0.001243(-2.9)
64(128 2.1)     - 0.005457(-2.3)
128(256 2.4)     - 0.026729(-1.6)
256(512 2.7)     - 0.126498(-0.9)
512(1024 3.0)     - 0.545565(-0.3)
1024(2048 3.3)     - 2.884641(0.5)
2048(4096 3.6)     - 12.263112(1.1)

 */

// A grid is formed by the intersection of a set of vertical and horizontal lines.
impl LinesNetTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver) {
        let subj_paths = Util::many_lines_x(20, n);
        let clip_paths = Util::many_lines_y(20, n);

        let it_count = ((500.0 / (n as f64)) as usize).max(1);
        let sq_it_count= it_count * it_count;

        let start = Instant::now();

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