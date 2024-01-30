use std::time::Instant;
use i_overlay::bool::fill_rule::FillRule;
use i_overlay::bool::overlay_rule::OverlayRule;
use i_overlay::layout::overlay::{Overlay, ShapeType};
use crate::test::util::Util;

pub(crate) struct NestedSquaresTest;

// Xor:
//    1000  - 0.019245761
//   10000  - 0.927409649
//  100000  - 77.99924515

// Union:
//    1000  - 0.018955962
//   10000  - 0.937482139
//  100000  - 77.990919978

// A series of concentric squares, each progressively larger than the last.
impl NestedSquaresTest {
    pub(crate) fn run(n: usize, rule: OverlayRule) {
        println!("Start NestedSquares Test {:?}", rule);
        let b = 4;
        let subj_paths = Util::concentric_squares(b, b, n);
        let clip_paths = Util::concentric_squares(b / 2, b, n);

        let start = Instant::now();

        let mut overlay = Overlay::new(8 * n * n);
        overlay.add_paths(&subj_paths, ShapeType::Subject);
        overlay.add_paths(&clip_paths, ShapeType::Clip);

        let graph = overlay.build_graph(FillRule::NonZero);
        let result = graph.extract_shapes(rule);

        assert!(!result.is_empty());

        let duration = start.elapsed();
        println!("Count: {:?}, time: {:?}", n, duration);
    }
}