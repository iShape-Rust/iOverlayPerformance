use std::time::Instant;
use i_float::fix_vec::FixVec;
use i_overlay::bool::fill_rule::FillRule;
use i_overlay::bool::overlay_rule::OverlayRule;
use i_overlay::layout::overlay::{Overlay, ShapeType};
use crate::test::util::Util;

pub(crate) struct CheckerboardTest;

// Union:
//  25  - 0.010584694
//  50  - 0.042548462
//  100 - 0.181674886
//  200 - 0.757548194
//  400 - 2.801719355
//  800 - 11.415289298
// 1600 - 47.940686084

// Xor:
//  25  - 0.010604308
//  50  - 0.045865777
//  100 - 0.195157346
//  200 - 0.797804374
//  400 - 2.998709203
//  800 - 11.994348965
// 1600 - 51.169205573

// A grid of overlapping squares forming a simple checkerboard pattern.
impl CheckerboardTest {
    pub(crate) fn run(n: usize, rule: OverlayRule) {
        println!("Start Checkerboard Test {:?}", rule);
        let subj_paths = Util::many_squares(FixVec::new(0, 0), 20, 30, n);
        let clip_paths = Util::many_squares(FixVec::new(15, 15), 20, 30, n - 1);

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