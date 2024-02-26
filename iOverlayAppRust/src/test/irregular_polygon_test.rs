use std::f64::consts::PI;
use std::time::Instant;
use i_overlay::bool::fill_rule::FillRule;
use i_overlay::bool::overlay_rule::OverlayRule;
use i_overlay::layout::overlay::{Overlay, ShapeType};
use crate::test::util::Util;

pub(crate) struct IrregularPolygonTest;

// 100_000 - 0.017672999
// 200_000 - 0.048966016
// 300_000 - 0.09378231
// 400_000 - 4.907101405

// Two irregular self-intersecting polygons are generated, the vertices of which are defined by a fixed radius and angle.
impl IrregularPolygonTest {
    pub(crate) fn run(n: usize, rule: OverlayRule) {
        println!("Start IrregularPolygon Test 2");
        let subj_path = Util::irregular_polygon(1000.0, 0.0, n);
        let clip_path = Util::irregular_polygon(1000.0, 0.5 * PI, n);

        let start = Instant::now();

        let mut overlay = Overlay::new(n);
        overlay.add_path(&subj_path, ShapeType::Subject);
        overlay.add_path(&clip_path, ShapeType::Clip);

        let graph = overlay.build_graph(FillRule::NonZero);
        let result = graph.extract_shapes(rule);

        assert!(!result.is_empty());

        let duration = start.elapsed();
        println!("Count: {:?}, time: {:?}", n, duration);
    }
}