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

make_day!(day01, day02);
