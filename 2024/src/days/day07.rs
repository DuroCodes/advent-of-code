use itertools::Itertools;

#[derive(Debug)]
pub struct Equation {
    value: i64,
    nums: Vec<i64>,
}

#[derive(Clone, Copy)]
enum Op {
    Add,
    Multiply,
    Concat,
}

fn evaluate(numbers: &[i64], operators: &Vec<&Op>) -> i64 {
    std::iter::zip(operators, &numbers[1..]).fold(numbers[0], |acc, (op, &num)| match op {
        Op::Add => acc + num,
        Op::Multiply => acc * num,
        Op::Concat => format!("{acc}{num}").parse().unwrap(),
    })
}

fn combinations(equation: &Equation, operators: &[Op]) -> bool {
    (0..equation.nums.len() - 1)
        .map(|_| operators)
        .multi_cartesian_product()
        .any(|ops| evaluate(&equation.nums, &ops) == equation.value)
}

pub fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (value, nums) = line.split(':').collect_tuple().unwrap();
            Equation {
                value: value.trim().parse().unwrap(),
                nums: nums
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

pub fn part1(input: &Vec<Equation>) -> String {
    input
        .iter()
        .filter(|eq| combinations(eq, &[Op::Add, Op::Multiply]))
        .map(|eq| eq.value)
        .sum::<i64>()
        .to_string()
}

pub fn part2(input: &Vec<Equation>) -> String {
    input
        .iter()
        .filter(|eq| combinations(eq, &[Op::Add, Op::Multiply, Op::Concat]))
        .map(|eq| eq.value)
        .sum::<i64>()
        .to_string()
}

#[test]
fn test_day07() {
    let input = parse(
        "190: 10 19
         3267: 81 40 27
         83: 17 5
         156: 15 6
         7290: 6 8 6 15
         161011: 16 10 13
         192: 17 8 14
         21037: 9 7 18 13
         292: 11 6 16 20",
    );
    assert_eq!(part1(&input), "3749");
    assert_eq!(part2(&input), "11387");
}
