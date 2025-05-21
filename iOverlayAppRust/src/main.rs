use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::solver::{Solver, MultithreadOptions, Precision, Strategy};
use crate::test::test_0_checkerboard::CheckerboardTest;
use crate::test::test_1_not_overlap::NotOverlapTest;
use crate::test::test_2_lines_net::LinesNetTest;
use crate::test::test_3_spiral::SpiralTest;
use crate::test::test_4_windows::WindowsTest;
use crate::test::test_5_nested_squares::CrossTest;
use crate::test::test_6_corrosion::CorrosionTest;
use crate::test::test_7_concentric::ConcentricTest;
use crate::test::test_8_wind_mill::WindMillTest;
use crate::util::args::EnvArgs;


mod test;
mod util;

fn main() {
    #[cfg(debug_assertions)]
    {
        debug_run();
    }

    #[cfg(not(debug_assertions))]
    {

        release_run();
    }
}

#[cfg(debug_assertions)]
fn debug_run() {
    // CheckerboardTest::run(10, OverlayRule::Xor, Default::default(), 1000.0);
    CorrosionTest::run(1, OverlayRule::Difference, Default::default(), 100.0);
}


fn release_run() {
    let args = EnvArgs::new();
    let multithreading = if args.get_bool("multithreading") {
        Some(MultithreadOptions::default())
    } else {
        None
    };

    let solver = Solver { strategy: Strategy::Auto, precision: Precision::HIGH, multithreading};

    if args.get_bool("complex") {
        complex_run(solver, args);
    } else {
        single_run(solver, args);
    }
}

fn complex_run(solver: Solver, args: EnvArgs) {
    let test = args.get_usize("test");
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
            run_test_3();
        }
        4 => {
            run_test_4(solver);
        }
        5 => {
            run_test_5(solver);
        }
        6 => {
            run_test_6(solver);
        }
        7 => {
            run_test_7(solver);
        }
        8 => {
            run_test_8(solver);
        }
        _ => {
            println!("Test is not found");
        }
    }
}
fn single_run(solver: Solver, args: EnvArgs) {
    let count = args.get_usize("count");
    let test = args.get_usize("test");
    match test {
        0 => {
            CheckerboardTest::run(count, OverlayRule::Xor, solver, 1.0);
        }
        1 => {
            NotOverlapTest::run(count, OverlayRule::Union, solver, 1.0);
        }
        2 => {
            LinesNetTest::run(count, OverlayRule::Intersect, solver, 1.0)
        }
        3 => {
            SpiralTest::run(count, 100.0);
        }
        4 => {
            WindowsTest::run(count, OverlayRule::Difference, solver, 1.0);
        }
        5 => {
            CrossTest::run(count, OverlayRule::Xor, solver, 1.0);
        }
        6 => {
            CorrosionTest::run(count, OverlayRule::Difference, solver, 1.0);
        }
        7 => {
            ConcentricTest::run(count, OverlayRule::Intersect, solver, 1.0);
        }
        8 => {
            ConcentricTest::run(count, OverlayRule::Intersect, solver, 1.0);
        }
        _ => {
            println!("Test is not found");
        }
    }
}

fn run_test_0(solver: Solver) {
    println!("run Checkerboard test");
    for i in 1..12 {
        let n = 1 << i;
        CheckerboardTest::run(n, OverlayRule::Xor, solver, 1000.0)
    }
}

fn run_test_1(solver: Solver) {
    println!("run NotOverlap test");
    for i in 1..12 {
        let n = 1 << i;
        NotOverlapTest::run(n, OverlayRule::Xor, solver, 1000.0)
    }
}

fn run_test_2(solver: Solver) {
    println!("run LinesNet test");
    for i in 1..12 {
        let n = 1 << i;
        LinesNetTest::run(n, OverlayRule::Intersect, solver, 500.0)
    }
}

fn run_test_3() {
    println!("run Spiral test");
    for i in 1..21 {
        let n = 1 << i;
        SpiralTest::run(n, 1000.0)
    }
}

fn run_test_4(solver: Solver) {
    println!("run Windows test");
    for i in 1..12 {
        let n = 1 << i;
        WindowsTest::run(n, OverlayRule::Difference, solver, 500.0)
    }
}

fn run_test_5(solver: Solver) {
    println!("run NestedSquares test");
    for i in 1..19 {
        let n = 1 << i;
        CrossTest::run(n, OverlayRule::Xor, solver, 500.0)
    }
}

fn run_test_6(solver: Solver) {
    println!("run Corrosion test");
    let mut n = 1;
    for _ in 1..12 {
        CorrosionTest::run(n, OverlayRule::Difference, solver, 100.0);
        n = n << 1;
    }
}

fn run_test_7(solver: Solver) {
    println!("run Concentric test");
    let mut n = 1;
    for _ in 1..12 {
        ConcentricTest::run(n, OverlayRule::Intersect, solver, 100.0);
        n = n << 1;
    }
}

fn run_test_8(solver: Solver) {
    println!("run WindMill test");
    let mut n = 1;
    for _ in 1..12 {
        WindMillTest::run(n, OverlayRule::Difference, solver, 100.0);
        n = n << 1;
    }
    WindMillTest::validate(100, OverlayRule::Difference, solver);
}