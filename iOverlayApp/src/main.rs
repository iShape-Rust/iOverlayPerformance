use std::env;
use std::collections::HashMap;
use i_overlay::bool::overlay_rule::OverlayRule;
use crate::test::concentric_squares_test::ConcentricSquaresTest;
use crate::test::lines_test::LinesTest;
use crate::test::no_overlap_test::NoOverlapTest;

use crate::test::squares_test::SquaresTest;
use crate::test::random_polygons_test::RandomPolygonsTest;
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
            SquaresTest::run(count, OverlayRule::Union);
        }
        1 => {
            SquaresTest::run(count, OverlayRule::Xor);
        }
        2 => {
            LinesTest::run(count, OverlayRule::Union)
        }
        3 => {
            LinesTest::run(count, OverlayRule::Xor)
        }
        4 => {
            NoOverlapTest::run(count);
        }
        5 => {
            RandomPolygonsTest::run(count);
        }
        6 => {
            WindowsTest::run(count, OverlayRule::Difference);
        }
        7 => {
            WindowsTest::run(count, OverlayRule::Union);
        }
        8 => {
            ConcentricSquaresTest::run(count, OverlayRule::Xor);
        }
        9 => {
            ConcentricSquaresTest::run(count, OverlayRule::Union);
        }
        _ => {
            println!("Test is not found");
        }
    }

    // RandomPolygonsTest::run(10100);
}
