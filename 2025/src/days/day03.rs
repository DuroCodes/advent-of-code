use itertools::Itertools;

pub fn parse(input: &str) -> Vec<String> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().to_string())
        .collect()
}

fn bank_digits(bank: &str) -> Vec<u32> {
    bank.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>()
}

fn max_joltage(bank: &str) -> u32 {
    let digits = bank_digits(bank);

    (0..digits.len())
        .combinations(2)
        .map(|indices| digits[indices[0]] * 10 + digits[indices[1]])
        .max()
        .unwrap_or(0)
}

pub fn part1(input: &[String]) -> String {
    input
        .iter()
        .map(|bank| max_joltage(bank))
        .sum::<u32>()
        .to_string()
}

fn max_joltage_12(bank: &str) -> u64 {
    let digits = bank_digits(bank);
    let batteries = 12;

    (0..batteries)
        .scan(-1i32, |last_idx, pos| {
            let needed = batteries - pos - 1;
            let start = (*last_idx + 1) as usize;
            let end = digits.len() - needed;

            let (max_digit, max_idx) = (start..end)
                .map(|i| (digits[i], i))
                .max_by_key(|&(d, i)| (d, -(i as i32)))
                .unwrap();

            *last_idx = max_idx as i32;
            Some(max_digit)
        })
        .fold(0u64, |acc, d| acc * 10 + d as u64)
}

pub fn part2(input: &[String]) -> String {
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
