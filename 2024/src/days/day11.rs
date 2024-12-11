use itertools::Itertools;
use std::collections::HashMap;

pub fn parse(input: &str) -> Vec<String> {
    input.split_whitespace().map(String::from).collect()
}

fn transform_stone(num: i64) -> Vec<i64> {
    match num {
        0 => vec![1],
        n => {
            let s = n.to_string();
            if s.len() % 2 == 0 {
                s.chars()
                    .chunks(s.len() / 2)
                    .into_iter()
                    .map(|c| {
                        c.collect::<String>()
                            .trim_start_matches('0')
                            .parse()
                            .unwrap_or(0)
                    })
                    .collect()
            } else {
                vec![n * 2024]
            }
        }
    }
}

fn simulate_blinks(stones: &[String], blinks: usize) -> usize {
    let mut stones: HashMap<_, usize> = stones
        .iter()
        .filter_map(|s| s.parse().ok())
        .map(|n| (n, 1))
        .into_group_map()
        .into_iter()
        .map(|(k, v)| (k, v.len()))
        .collect();

    for _ in 0..blinks {
        stones = stones
            .into_iter()
            .flat_map(|(n, c)| transform_stone(n).into_iter().map(move |x| (x, c)))
            .into_group_map()
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().sum()))
            .collect();
    }
    stones.values().sum()
}

pub fn part1(input: &Vec<String>) -> String {
    simulate_blinks(input, 25).to_string()
}

pub fn part2(input: &Vec<String>) -> String {
    simulate_blinks(input, 75).to_string()
}

#[test]
fn test_day11() {
    let input = parse("125 17");
    assert_eq!(part1(&input), "55312");
    assert_eq!(part2(&input), "65601038650482");
}
