use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{Overlay, ShapeType};
use i_overlay::core::overlay_rule::OverlayRule;
use crate::test::util::Util;

pub(crate) struct LinesNetTest;

// Union:
//   25  - 0.003469159
//   50  - 0.016720316
//  100  - 0.100604056
//  200  - 0.63058424
//  300  - 1.832248187
//  400  - 4.158703557
//  500  - 7.595058528
//  600 - 13.479658167
//  700 - 21.908887535
//  800 - 33.237526606
//  900 - 47.216105921
// 1000 - 64.886755957

// A grid is formed by the intersection of a set of vertical and horizontal lines.
impl LinesNetTest {
    pub(crate) fn run(n: usize, rule: OverlayRule) {
        println!("Start LinesNet Test {:?}", rule);
        let subj_paths = Util::many_lines_x(20, n);
        let clip_paths = Util::many_lines_y(20, n);

        let start = Instant::now();

        let mut overlay = Overlay::new(8 * n);
        overlay.add_paths(&subj_paths, ShapeType::Subject);
        overlay.add_paths(&clip_paths, ShapeType::Clip);

        let graph = overlay.build_graph(FillRule::NonZero);
        let result = graph.extract_shapes(rule);

        debug_assert!(!result.is_empty());

        let duration = start.elapsed();
        println!("Count: {:?}, time: {:?}", n, duration);
    }
}