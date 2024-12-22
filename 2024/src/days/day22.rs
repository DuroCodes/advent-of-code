use itertools::iproduct;
use rayon::prelude::*;

pub fn parse(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn next_secret(current: u64) -> u64 {
    let mut next = current;
    next ^= next * 64;
    next %= 16777216;
    next ^= next / 32;
    next %= 16777216;
    next ^= next * 2048;
    next % 16777216
}

fn nth_secret(initial: u64, n: usize) -> u64 {
    (0..n).fold(initial, |current, _| next_secret(current))
}

fn get_price(secret: u64) -> i32 {
    (secret % 10) as i32
}

fn price_changes(initial: u64, n: usize) -> Vec<i32> {
    std::iter::successors(Some(initial), |&current| Some(next_secret(current)))
        .take(n + 1)
        .map(get_price)
        .collect::<Vec<_>>()
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect()
}

pub fn part1(input: &Vec<u64>) -> String {
    input
        .iter()
        .map(|&initial| nth_secret(initial, 2000))
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: &Vec<u64>) -> String {
    let changes: Vec<(usize, Vec<i32>)> = input
        .par_iter()
        .enumerate()
        .map(|(idx, &initial)| (idx, price_changes(initial, 2000)))
        .collect();

    iproduct!(-9..=9, -9..=9, -9..=9, -9..=9)
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|(a, b, c, d)| {
            changes
                .iter()
                .map(|(idx, change)| {
                    change
                        .windows(4)
                        .enumerate()
                        .find(|(_, window)| window == &[a, b, c, d])
                        .map(|(pos, _)| get_price(nth_secret(input[*idx], pos + 4)))
                        .unwrap_or(0)
                })
                .sum::<i32>()
        })
        .max()
        .unwrap()
        .to_string()
}

#[test]
fn test_day22() {
    let input = parse("1\n10\n100\n2024");
    assert_eq!(part1(&input), "37327623");
    assert_eq!(part2(&input), "24");
}
