use std::time::Instant;
use i_float::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{Overlay, ShapeType};
use i_overlay::core::overlay_rule::OverlayRule;
use crate::test::util::Util;

pub(crate) struct NotOverlapTest;

// 2
// Union:

//   3(3.2)        - 0.000009(-5.0)
//   6(5.2)        - 0.000035(-4.5)
//  12(7.2)        - 0.000165(-3.8)
//  25(9.3)        - 0.000845(-3.1)
//  50(11.3)       - 0.003907(-2.4)
//  100(13.3)      - 0.018785(-1.7)
//  200(15.3)      - 0.087480(-1.1)
//  400(17.3)      - 0.392822(-0.4)
//  800(19.3)      - 1.697748(0.2)
// 1600(21.3)      - 7.326802(0.9)
// 3200(23.3)      - 29.108937(1.5)
// 6400(25.3)     - 130.862234(2.1)

// A grid of not overlapping squares.
impl NotOverlapTest {
    pub(crate) fn run(n: usize) {
        println!("Start NotOverlap Test");
        let subj_paths = Util::many_squares(IntPoint::new(0, 0), 20, 30, n);

        let start = Instant::now();

        let it_count = ((1000.0 / (n as f64)) as usize).max(1);
        let it_count= it_count * it_count;
        for _ in 0..it_count {
            let mut overlay = Overlay::new(4 * n * n);
            overlay.add_paths(&subj_paths, ShapeType::Subject);

            let graph = overlay.build_graph(FillRule::NonZero);
            _ = graph.extract_shapes(OverlayRule::Subject);
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / it_count as f64;

        let count = n * n;
        let count_log = (count as f64).log2();
        let time_log = time.log10();

        println!("{}({:.1})     - {:.6}({:.1})", n, count_log, time, time_log);
    }
}