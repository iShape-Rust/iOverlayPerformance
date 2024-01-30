use std::env;
use std::collections::HashMap;
use i_overlay::bool::overlay_rule::OverlayRule;
use crate::test::nested_squares_test::NestedSquaresTest;
use crate::test::lines_net_test::LinesNetTest;
use crate::test::not_overlap_test::NotOverlapTest;

use crate::test::checkerboard_test::CheckerboardTest;
use crate::test::irregular_polygon_test::IrregularPolygonTest;
use crate::test::windows_test::WindowsTest;

mod test;

fn main() {
    let args = env::args();
    let mut args_iter = args.peekable();
    let mut args_map = HashMap::new();

    while let Some(arg) = args_iter.next() {
        if arg.starts_with("--") {
            let key = arg.trim_start_matches("--").to_owned();
            // If the next argument is also a key, store a boolean flag; otherwise, store the value.
            let value = if args_iter.peek().map_or(false, |a| a.starts_with("--")) {
                "true".to_string()
            } else {
                args_iter.next().unwrap()
            };
            args_map.insert(key, value);
        }
    }

    let test_key = args_map.get("test").expect("Test number is not set");
    let count_key = args_map.get("count").expect("Count is not set");

    let test: usize = test_key.parse().expect("Unable to parse test as an integer");
    let count: usize = count_key.parse().expect("Unable to parse count as an integer");

    match test {
        0 => {
            CheckerboardTest::run(count, OverlayRule::Xor);
        }
        1 => {
            LinesNetTest::run(count, OverlayRule::Union)
        }
        2 => {
            NotOverlapTest::run(count);
        }
        3 => {
            IrregularPolygonTest::run(count);
        }
        4 => {
            WindowsTest::run(count, OverlayRule::Difference);
        }
        5 => {
            NestedSquaresTest::run(count, OverlayRule::Intersect);
        }
        _ => {
            println!("Test is not found");
        }
    }

    // RandomPolygonsTest::run(10100);
}
