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

2(4 0.6)     - 0.000008(-5.1)
4(8 0.9)     - 0.000025(-4.6)
8(16 1.2)     - 0.000095(-4.0)
16(32 1.5)     - 0.000709(-3.1)
32(64 1.8)     - 0.002847(-2.5)
64(128 2.1)     - 0.013217(-1.9)
128(256 2.4)     - 0.034898(-1.5)
256(512 2.7)     - 0.149629(-0.8)
512(1024 3.0)     - 0.680594(-0.2)
1024(2048 3.3)     - 2.818377(0.4)
2048(4096 3.6)     - 11.703711(1.1)


multithreading off

2(4 0.6)     - 0.000007(-5.1)
4(8 0.9)     - 0.000023(-4.6)
8(16 1.2)     - 0.000087(-4.1)
16(32 1.5)     - 0.000672(-3.2)
32(64 1.8)     - 0.002716(-2.6)
64(128 2.1)     - 0.012761(-1.9)
128(256 2.4)     - 0.037645(-1.4)
256(512 2.7)     - 0.192885(-0.7)
512(1024 3.0)     - 0.792870(-0.1)
1024(2048 3.3)     - 3.328241(0.5)
2048(4096 3.6)     - 13.678483(1.1)

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