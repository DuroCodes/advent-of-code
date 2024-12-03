use itertools::Itertools;

pub fn parse(input: &str) -> Vec<(u32, u32)> {
    let (first, second): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|part| part.parse().unwrap())
                .collect_tuple::<(u32, u32)>()
                .unwrap()
        })
        .unzip();

    first
        .into_iter()
        .sorted()
        .zip(second.into_iter().sorted())
        .collect()
}

pub fn part1(input: &[(u32, u32)]) -> String {
    input
        .iter()
        .map(|&(a, b)| a.abs_diff(b))
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: &[(u32, u32)]) -> String {
    input
        .iter()
        .map(|&(a, _)| a * input.iter().filter(|&&(_, b)| b == a).count() as u32)
        .sum::<u32>()
        .to_string()
}

#[test]
fn test_day01() {
    let input = parse(
        "3   4
                4   3
                2   5
                1   3
                3   9
                3   3",
    );
    assert_eq!(part1(&input), "11");
    assert_eq!(part2(&input), "31");
}
