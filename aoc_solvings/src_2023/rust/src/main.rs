mod days;
mod etc;
mod parser;

use std::time::Instant;

use days::prelude as Days;
use etc::SolutionPair;
use parser::prelude::*;
use parser::Arguments;

fn solve_day(day: u16) -> fn(Option<&str>) -> SolutionPair {
    match day {
        1 => Days::day01::solve,
        // 2 => day_2::solve(),
        // 3 => day_3::solve(),
        // 4 => day_4::solve(),
        // 5 => day_5::solve(),
        // 6 => day_6::solve(),
        // 7 => day_7::solve(),
        // 8 => day_8::solve(),
        // 9 => day_9::solve(),
        // 10 => day_10::solve(),
        // 11 => day_11::solve(),
        // 12 => day_12::solve(),
        // 13 => day_13::solve(),
        // 14 => day_14::solve(),
        // 15 => day_15::solve(),
        // 16 => day_16::solve(),
        // 17 => day_17::solve(),
        // 18 => day_18::solve(),
        // 19 => day_19::solve(),
        // 20 => day_20::solve(),
        // 21 => day_21::solve(),
        // 22 => day_22::solve(),
        // 23 => day_23::solve(),
        // 24 => day_24::solve(),
        // 25 => day_25::solve(),
        _ => panic!("Day {day} is not implemented yet"),
    }
}

#[allow(clippy::cast_precision_loss)]
fn main() {
    let args = Arguments::parse();
    let mut runtime = 0.0;
    for day in args.days {
        let func = solve_day(day);
        let time = Instant::now();
        // this function takes an optional input path to the input file
        // otherwise it uses the hard coded path of the local input file
        let (p1, p2) = func(None);
        let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;

        println!("\n=== Day {day:02} ===");
        println!("  - Part 1: {p1}");
        println!("  - Part 2: {p2}");
        println!("  - Elapsed: {elapsed_ms:.4} ms");

        runtime += elapsed_ms;
    }
    println!("Total runtime: {runtime:.4} ms");
}
