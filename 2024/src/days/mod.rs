pub mod day01;

pub trait Day<T> {
    fn parse(&self, input: &str) -> T;
    fn part1(&self, input: &T) -> String;
    fn part2(&self, input: &T) -> String;
}
