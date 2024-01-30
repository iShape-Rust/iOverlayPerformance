use std::time::Instant;
use i_float::fix_vec::FixVec;
use i_overlay::bool::fill_rule::FillRule;
use i_overlay::bool::overlay_rule::OverlayRule;
use i_overlay::layout::overlay::{Overlay, ShapeType};
use crate::test::util::Util;

pub(crate) struct NoOverlapTest;

//  25  - 0.00191984
//  50  - 0.010274676
//  100 - 0.044639658
//  200 - 0.23951921
//  400 - 1.321968217
//  800 - 5.293362805
// 1600 - 21.095707913

impl NoOverlapTest {
    pub(crate) fn run(n: usize) {
        println!("Start NoOverlap Test");
        let subj_paths = Util::many_squares(FixVec::new(0, 0), 20, 30, n);

        let start = Instant::now();

        let mut overlay = Overlay::new(4 * n * n); // Assuming Overlay is defined elsewhere
        overlay.add_paths(&subj_paths, ShapeType::Subject); // Assuming method and enum names

        let graph = overlay.build_graph(FillRule::NonZero); // Assuming FillRule enum
        let result = graph.extract_shapes(OverlayRule::Subject); // Assuming method and types

        assert!(!result.is_empty());

        let duration = start.elapsed();
        println!("Count: {:?}, time: {:?}", n, duration);
    }


}