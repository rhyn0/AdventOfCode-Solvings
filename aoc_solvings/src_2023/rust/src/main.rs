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
        2 => Days::day02::solve,
        3 => Days::day03::solve,
        4 => Days::day04::solve,
        5 => Days::day05::solve,
        6 => Days::day06::solve,
        7 => Days::day07::solve,
        8 => Days::day08::solve,
        9 => Days::day09::solve,
        10 => Days::day10::solve,
        11 => Days::day11::solve,
        12 => Days::day12::solve,
        13 => Days::day13::solve,
        // 14 => Days::day14::solve,
        // 15 => Days::day15::solve,
        // 16 => Days::day16::solve,
        // 17 => Days::day17::solve,
        // 18 => Days::day18::solve,
        // 19 => Days::day19::solve,
        // 20 => Days::day20::solve,
        // 21 => Days::day21::solve,
        // 22 => Days::day22::solve,
        // 23 => Days::day23::solve,
        // 24 => Days::day24::solve,
        // 25 => Days::day25::solve,
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
