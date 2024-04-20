use std::time::Instant;
use i_float::point::IntPoint;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{Overlay, ShapeType};
use i_overlay::core::overlay_rule::OverlayRule;
use crate::test::util::Util;

pub(crate) struct CheckerboardTest;

// Xor:
//  25  - 0.010604308
//  50  - 0.045865777
//  100 - 0.195157346
//  200 - 0.797804374
//  400 - 2.998709203
//  800 - 11.994348965
// 1600 - 51.169205573

// Xor:
//  25  - 0.0087
//  50  - 0.038597548
//  100 - 0.163943776
//  200 - 0.673153318
//  400 - 2.733037367
//  800 - 11.25430898
// 1600 - 47.319312662

// A grid of overlapping squares forming a simple checkerboard pattern.
impl CheckerboardTest {
    pub(crate) fn run(n: usize, rule: OverlayRule) {
        println!("Start Checkerboard Test {:?}", rule);
        let subj_paths = Util::many_squares(IntPoint::new(0, 0), 20, 30, n);
        let clip_paths = Util::many_squares(IntPoint::new(15, 15), 20, 30, n - 1);

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