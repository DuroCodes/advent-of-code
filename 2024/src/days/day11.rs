use itertools::Itertools;
use std::collections::HashMap;

pub fn parse(input: &str) -> Vec<String> {
    input.split_whitespace().map(String::from).collect()
}

fn transform(num: i64) -> Vec<i64> {
    if num == 0 {
        return vec![1];
    }

    let s = num.to_string();
    match s.len() % 2 {
        0 => s
            .chars()
            .chunks(s.len() / 2)
            .into_iter()
            .map(|c| c.collect::<String>().parse().unwrap())
            .collect(),
        _ => vec![num * 2024],
    }
}

fn simulate(stones: &[String], blinks: usize) -> usize {
    let mut counts: HashMap<_, usize> = stones
        .iter()
        .filter_map(|s| s.parse().ok())
        .map(|n| (n, 1))
        .collect();

    for _ in 0..blinks {
        counts = counts
            .into_iter()
            .flat_map(|(n, c)| transform(n).into_iter().map(move |x| (x, c)))
            .into_group_map()
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().sum()))
            .collect();
    }

    counts.values().sum()
}

pub fn part1(input: &Vec<String>) -> String {
    simulate(input, 25).to_string()
}

pub fn part2(input: &Vec<String>) -> String {
    simulate(input, 75).to_string()
}

#[test]
fn test_day11() {
    let input = parse("125 17");
    assert_eq!(part1(&input), "55312");
    assert_eq!(part2(&input), "65601038650482");
}
