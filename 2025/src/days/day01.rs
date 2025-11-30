pub fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(input: &[u32]) -> String {
    "".to_string()
}

pub fn part2(input: &[u32]) -> String {
    "".to_string()
}

#[test]
fn test_day01() {
    let input = parse("");
    assert_eq!(part1(&input), "");
    assert_eq!(part2(&input), "");
}
