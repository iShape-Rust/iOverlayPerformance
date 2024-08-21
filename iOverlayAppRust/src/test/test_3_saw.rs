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

2(4 0.6)     - 0.000008(-5.1)
4(8 0.9)     - 0.000028(-4.6)
8(16 1.2)     - 0.000131(-3.9)
16(32 1.5)     - 0.000613(-3.2)
32(64 1.8)     - 0.002923(-2.5)
64(128 2.1)     - 0.012620(-1.9)
128(256 2.4)     - 0.054968(-1.3)
256(512 2.7)     - 0.237621(-0.6)
512(1024 3.0)     - 1.009563(0.0)
1024(2048 3.3)     - 4.115561(0.6)
2048(4096 3.6)     - 17.882025(1.3)

multithreading off

2(4 0.6)     - 0.000008(-5.1)
4(8 0.9)     - 0.000028(-4.6)
8(16 1.2)     - 0.000130(-3.9)
16(32 1.5)     - 0.000730(-3.1)
32(64 1.8)     - 0.002874(-2.5)
64(128 2.1)     - 0.012346(-1.9)
128(256 2.4)     - 0.058456(-1.2)
256(512 2.7)     - 0.274925(-0.6)
512(1024 3.0)     - 1.058200(0.0)
1024(2048 3.3)     - 4.741841(0.7)
2048(4096 3.6)     - 19.849207(1.3)

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