use std::time::Instant;
use i_float::fix_vec::FixVec;
use i_overlay::bool::fill_rule::FillRule;
use i_overlay::bool::overlay_rule::OverlayRule;
use i_overlay::layout::overlay::{Overlay, ShapeType};
use crate::test::util::Util;

pub(crate) struct WindowsTest;

// Difference:
//  25  - 0.004407178
//  50  - 0.02003032
//  100 - 0.098023803
//  200 - 0.487250454
//  400 - 1.889019334
//  800 - 7.715119585
// 1600 - 31.673431788

// Union:
//  25  - 0.003863819
//  50  - 0.018456852
//  100 - 0.093255047
//  200 - 0.467733968
//  400 - 1.821580309
//  800 - 7.275199854
// 1600 - 30.257031037

// A grid of square frames, each with a smaller square cutout in the center.
impl WindowsTest {
    pub(crate) fn run(n: usize, rule: OverlayRule) {
        println!("Start Windows Test {:?}", rule);

        let offset = 30;
        let x = (n as i64) * offset / 2;
        let origin = FixVec::new(-x, -x);
        let (subj_paths, clip_paths) = Util::many_windows(origin, 20, 10, offset, n);

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