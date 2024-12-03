pub mod day01;
pub mod day02;
pub mod day03;

type DaySolution = (fn(&str) -> String, fn(&str) -> String);

macro_rules! solutions {
    ($($day:ident),*) => {
        pub const SOLUTIONS: &[DaySolution] = &[$((
            |input| $day::part1(&$day::parse(input)),
            |input| $day::part2(&$day::parse(input))
        ),)*];
    }
}

solutions!(day01, day02, day03);
