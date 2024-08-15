use std::env;
use std::collections::HashMap;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::{MultithreadOptions, Solver, Strategy};
use crate::test::test_0_checkerboard::CheckerboardTest;
use crate::test::test_1_not_overlap::NotOverlapTest;
use crate::test::test_2_lines_net::LinesNetTest;
use crate::test::test_3_saw_test::SawTest;
use crate::test::test_4_windows::WindowsTest;
use crate::test::test_5_nested_squares::NestedSquaresTest;

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

    #[cfg(debug_assertions)]
    {
        if args_map.is_empty() {
            args_map.insert("test".to_string(), "0".to_string());
            args_map.insert("count".to_string(), "3".to_string());
        }
    }

    let test_key = args_map.get("test").expect("Test number is not set");
    let count_key = args_map.get("count").expect("Count is not set");
    let multithreading_key = args_map.get("multithreading").expect("Multithreading is not set");

    let test: usize = test_key.parse().expect("Unable to parse test as an integer");
    let count: usize = count_key.parse().expect("Unable to parse count as an integer");
    let multithreading: bool = multithreading_key.parse().expect("Unable to parse multithreading as an boolean");

    let multithreading = if multithreading {
        Some(MultithreadOptions::default())
    } else {
        None
    };

    let solver = Solver { strategy: Strategy::Auto, multithreading};

    match test {
        0 => {
            CheckerboardTest::run(count, OverlayRule::Xor, solver);
        }
        1 => {
            NotOverlapTest::run(count, OverlayRule::Xor, solver);
        }
        2 => {
            LinesNetTest::run(count, OverlayRule::Intersect, solver)
        }
        3 => {
            SawTest::run(count, OverlayRule::Intersect, solver);
        }
        4 => {
            WindowsTest::run(count, OverlayRule::Difference, solver);
        }
        5 => {
            NestedSquaresTest::run(count, OverlayRule::Union, solver);
        }
        _ => {
            println!("Test is not found");
        }
    }
}
