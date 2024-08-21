use std::env;
use std::collections::HashMap;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::{MultithreadOptions, Solver, Strategy};
use crate::test::test_0_checkerboard::CheckerboardTest;
use crate::test::test_1_not_overlap::NotOverlapTest;
use crate::test::test_2_lines_net::LinesNetTest;
use crate::test::test_3_saw::SawTest;
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
            args_map.insert("multithreading".to_string(), "false".to_string());
            args_map.insert("complex".to_string(), "false".to_string());
            args_map.insert("test".to_string(), "0".to_string());
            args_map.insert("count".to_string(), "1000".to_string());
        }
    }

    let test_key = args_map.get("test").expect("Test number is not set");
    let multithreading_key = args_map.get("multithreading").expect("Multithreading is not set");
    let complex_key = args_map.get("complex").expect("Complex is not set");

    let test: usize = test_key.parse().expect("Unable to parse test as an integer");
    let multithreading: bool = multithreading_key.parse().expect("Unable to parse multithreading as an boolean");
    let complex: bool = complex_key.parse().expect("Unable to parse complex as an boolean");

    let multithreading = if multithreading {
        Some(MultithreadOptions::default())
    } else {
        None
    };

    let solver = Solver { strategy: Strategy::Auto, multithreading};

    if complex {
        match test {
            0 => {
                run_test_0(solver);
            }
            1 => {
                run_test_1(solver);
            }
            2 => {
                run_test_2(solver)
            }
            3 => {
                run_test_3(solver);
            }
            4 => {
                run_test_4(solver);
            }
            5 => {
                run_test_5(solver);
            }
            _ => {
                println!("Test is not found");
            }
        }
    } else {
        let count_key = args_map.get("count").expect("Count is not set");
        let count: usize = count_key.parse().expect("Unable to parse count as an integer");
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
}

fn run_test_0(solver: Solver) {
    println!("run Checkerboard test");
    for i in 1..12 {
        let n = 1 << i;
        CheckerboardTest::run(n, OverlayRule::Xor, solver)
    }
}

fn run_test_1(solver: Solver) {
    println!("run NotOverlap test");
    for i in 1..12 {
        let n = 1 << i;
        NotOverlapTest::run(n, OverlayRule::Xor, solver)
    }
}

fn run_test_2(solver: Solver) {
    println!("run LinesNet test");
    for i in 1..12 {
        let n = 1 << i;
        LinesNetTest::run(n, OverlayRule::Intersect, solver)
    }
}

fn run_test_3(solver: Solver) {
    println!("run Saw test");
    for i in 1..12 {
        let n = 1 << i;
        SawTest::run(n, OverlayRule::Intersect, solver)
    }
}

fn run_test_4(solver: Solver) {
    println!("run Windows test");
    for i in 1..12 {
        let n = 1 << i;
        WindowsTest::run(n, OverlayRule::Difference, solver)
    }
}

fn run_test_5(solver: Solver) {
    println!("run NestedSquares test");
    for i in 1..19 {
        let n = 1 << i;
        NestedSquaresTest::run(n, OverlayRule::Union, solver)
    }
}