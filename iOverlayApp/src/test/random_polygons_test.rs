use std::time::Instant;
use i_overlay::bool::fill_rule::FillRule;
use i_overlay::bool::overlay_rule::OverlayRule;
use i_overlay::layout::overlay::{Overlay, ShapeType};
use crate::test::util::Util;

pub(crate) struct RandomPolygonsTest;


// 100_000 - 0.017672999
// 200_000 - 0.048966016
// 300_000 - 0.09378231
// 400_000 - 4.907101405

impl RandomPolygonsTest {
    pub(crate) fn run(n: usize) {
        println!("Start RandomPolygons Test 2");
        let subj_paths = Util::random_polygon(1000.0, n);

        let start = Instant::now();

        let mut overlay = Overlay::new(n);
        overlay.add_path(&subj_paths, ShapeType::Subject);

        let graph = overlay.build_graph(FillRule::NonZero);
        let result = graph.extract_shapes(OverlayRule::Subject);

        assert!(!result.is_empty());

        let duration = start.elapsed();
        println!("Count: {:?}, time: {:?}", n, duration);
    }
}