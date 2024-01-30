use std::time::Instant;
use i_overlay::bool::fill_rule::FillRule;
use i_overlay::bool::overlay_rule::OverlayRule;
use i_overlay::layout::overlay::{Overlay, ShapeType};
use crate::test::util::Util;

pub(crate) struct  LinesTest;

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

// Xor:
//   25  - 0.003657718
//   50  - 0.01766465
//  100  - 0.101413229
//  200  - 0.64390637
//  300  - 1.877151657
//  400  - 4.309943373
//  500  - 7.829302811
//  600 - 13.860809147
//  700 - 22.323813061
//  800 - 33.593015232
//  900 - 47.947838463
// 1000 - 64.686463853

impl LinesTest {
    pub(crate) fn run(n: usize, rule: OverlayRule) {
        println!("Start Squares Test {:?}", rule);
        let subj_paths = Util::many_lines_x(20, n);
        let clip_paths = Util::many_lines_y(20, n);

        let start = Instant::now();

        let mut overlay = Overlay::new(8 * n);
        overlay.add_paths(&subj_paths, ShapeType::Subject);
        overlay.add_paths(&clip_paths, ShapeType::Clip);

        let graph = overlay.build_graph(FillRule::NonZero);
        let result = graph.extract_shapes(rule);

        assert!(!result.is_empty());

        let duration = start.elapsed();
        println!("Count: {:?}, time: {:?}", n, duration);
    }
}