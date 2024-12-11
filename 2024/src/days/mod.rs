pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;

type DaySolution = (fn(&str) -> String, fn(&str) -> String);

macro_rules! solutions {
    ($($day:ident),*) => {
        pub const SOLUTIONS: &[DaySolution] = &[$((
            |input| $day::part1(&$day::parse(input)),
            |input| $day::part2(&$day::parse(input))
        ),)*];
    }
}

solutions!(day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11);
