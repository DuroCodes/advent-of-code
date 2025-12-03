use itertools::Itertools;

pub fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn max_joltage(bank: &[u32]) -> u32 {
    (0..bank.len())
        .tuple_combinations()
        .map(|(i, j)| bank[i] * 10 + bank[j])
        .max()
        .unwrap_or(0)
}

pub fn part1(input: &[Vec<u32>]) -> String {
    input
        .iter()
        .map(|bank| max_joltage(bank))
        .sum::<u32>()
        .to_string()
}

fn max_joltage_12(bank: &[u32]) -> u64 {
    let batteries = 12;

    (0..batteries)
        .scan(-1i32, |last_idx, pos| {
            let needed = batteries - pos - 1;
            let start = (*last_idx + 1) as usize;
            let end = bank.len() - needed;

            let (max_digit, max_idx) = (start..end)
                .map(|i| (bank[i], i))
                .max_by_key(|&(d, i)| (d, -(i as i32)))
                .unwrap();

            *last_idx = max_idx as i32;
            Some(max_digit)
        })
        .fold(0u64, |acc, d| acc * 10 + d as u64)
}

pub fn part2(input: &[Vec<u32>]) -> String {
    input
        .iter()
        .map(|bank| max_joltage_12(bank))
        .sum::<u64>()
        .to_string()
}

#[test]
fn test_day03() {
    let input = parse("987654321111111\n811111111111119\n234234234234278\n818181911112111");
    assert_eq!(part1(&input), "357");
    assert_eq!(part2(&input), "3121910778619");
}
