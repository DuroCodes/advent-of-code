type DaySolution = (fn(&str) -> String, fn(&str) -> String);

macro_rules! make_day {
    ($($day:ident),*) => {
        $(
            pub mod $day;
        )*

        pub const SOLUTIONS: &[DaySolution] = &[
            $((
                |input| $day::part1(&$day::parse(input)),
                |input| $day::part2(&$day::parse(input))
            ),)*
        ];
    }
}

make_day!(
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24
);
