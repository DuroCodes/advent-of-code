pub mod day01;
pub mod day02;

pub trait Day<T> {
    fn parse(&self, input: &str) -> T;
    fn part1(&self, input: &T) -> String;
    fn part2(&self, input: &T) -> String;
}

pub enum Solution {
    Day01(day01::Day01),
    Day02(day02::Day02),
}

impl Solution {
    pub fn run(&self, input: &str) -> (String, String) {
        match self {
            Solution::Day01(day) => {
                let parsed = day.parse(input);
                (day.part1(&parsed), day.part2(&parsed))
            }
            Solution::Day02(day) => {
                let parsed = day.parse(input);
                (day.part1(&parsed), day.part2(&parsed))
            }
        }
    }
}
