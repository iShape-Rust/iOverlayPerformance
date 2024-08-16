use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::Overlay;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::Solver;
use crate::test::util::Util;

pub(crate) struct SawTest;

/*

// 3
// Intersection:


multithreading on

2(4 0.6)     - 0.000010(-5.0)
4(8 0.9)     - 0.000033(-4.5)
8(16 1.2)     - 0.000155(-3.8)
16(32 1.5)     - 0.000815(-3.1)
32(64 1.8)     - 0.003266(-2.5)
64(128 2.1)     - 0.013641(-1.9)
128(256 2.4)     - 0.059473(-1.2)
256(512 2.7)     - 0.277905(-0.6)
512(1024 3.0)     - 1.122181(0.1)
1024(2048 3.3)     - 4.628867(0.7)
2048(4096 3.6)     - 19.235394(1.3)


multithreading off

2(4 0.6)     - 0.000010(-5.0)
4(8 0.9)     - 0.000033(-4.5)
8(16 1.2)     - 0.000157(-3.8)
16(32 1.5)     - 0.000815(-3.1)
32(64 1.8)     - 0.003200(-2.5)
64(128 2.1)     - 0.013980(-1.9)
128(256 2.4)     - 0.062922(-1.2)
256(512 2.7)     - 0.298031(-0.5)
512(1024 3.0)     - 1.227951(0.1)
1024(2048 3.3)     - 5.309530(0.7)
2048(4096 3.6)     - 22.598605(1.4)

 */

// Two irregular self-intersecting polygons are generated, the vertices of which are defined by a fixed radius and angle.
impl SawTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, solver: Solver) {
        let subj_paths = Util::saw_lines_x(20, n);
        let clip_paths = Util::saw_lines_y(20, n);

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