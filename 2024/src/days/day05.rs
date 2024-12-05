use itertools::Itertools;
use std::cmp::Ordering::*;
use std::collections::HashSet;

pub type Rule = (u32, u32);

#[derive(Debug)]
pub struct Input {
    rules: Vec<Rule>,
    updates: Vec<Vec<u32>>,
}

pub fn parse(input: &str) -> Input {
    let (rules, updates) = input.split("\n\n").collect_tuple().unwrap();
    let rules = rules
        .lines()
        .filter_map(|line| line.split('|').map(str::trim).collect_tuple())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    let updates = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(str::trim)
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    Input { rules, updates }
}

fn is_valid(sequence: &[u32], rules: &[Rule]) -> bool {
    sequence
        .iter()
        .tuple_windows()
        .all(|(&a, &b)| !rules.iter().any(|&(x, y)| a == y && b == x))
}

fn sort(sequence: &[u32], rules: &[Rule]) -> Vec<u32> {
    let nums: HashSet<_> = sequence.iter().copied().collect();
    sequence
        .iter()
        .copied()
        .sorted_by(|&a, &b| {
            rules
                .iter()
                .filter(|&&(x, y)| nums.contains(&x) && nums.contains(&y))
                .find(|&&(x, y)| (a, b) == (y, x) || (a, b) == (x, y))
                .map_or(Equal, |&(x, _)| if x == a { Less } else { Greater })
        })
        .collect()
}

pub fn part1(input: &Input) -> String {
    input
        .updates
        .iter()
        .filter(|seq| is_valid(seq, &input.rules))
        .map(|seq| seq[seq.len() / 2])
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: &Input) -> String {
    input
        .updates
        .iter()
        .filter(|seq| !is_valid(seq, &input.rules))
        .map(|seq| sort(seq, &input.rules)[seq.len() / 2])
        .sum::<u32>()
        .to_string()
}

#[test]
fn test_day05() {
    let input = parse(
        "47|53
         97|13
         97|61
         97|47
         75|29
         61|13
         75|53
         29|13
         97|29
         53|29
         61|53
         97|53
         61|29
         47|13
         75|47
         97|75
         47|61
         75|61
         47|29
         75|13
         53|13
 
         75,47,61,53,29
         97,61,53,29,13
         75,29,13
         75,97,47,61,53
         61,13,29
         97,13,75,29,47",
    );
    assert_eq!(part1(&input), "143");
    assert_eq!(part2(&input), "123");
}
