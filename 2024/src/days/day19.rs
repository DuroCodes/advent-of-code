use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Input {
    patterns: Vec<String>,
    designs: Vec<String>,
}

pub fn parse(input: &str) -> Input {
    let (patterns, designs) = input.split("\n\n").collect_tuple().unwrap_or_default();

    Input {
        patterns: patterns.split(", ").map(str::to_string).collect(),
        designs: designs.lines().map(str::trim).map(str::to_string).collect(),
    }
}

fn can_make(design: &str, patterns: &[String]) -> bool {
    if design.is_empty() {
        return true;
    }

    patterns
        .iter()
        .filter(|pattern| design.starts_with(*pattern))
        .any(|pattern| can_make(&design[pattern.len()..], patterns))
}

fn arrangements(design: &str, patterns: &[String], memo: &mut HashMap<String, u64>) -> u64 {
    if design.is_empty() {
        return 1;
    }

    if let Some(&count) = memo.get(design) {
        return count;
    }

    let total = patterns
        .iter()
        .filter(|pattern| design.starts_with(*pattern))
        .map(|pattern| arrangements(&design[pattern.len()..], patterns, memo))
        .sum();

    memo.insert(design.to_string(), total);
    total
}

pub fn part1(input: &Input) -> String {
    let count = input
        .designs
        .iter()
        .filter(|design| can_make(design, &input.patterns))
        .count();

    count.to_string()
}

pub fn part2(input: &Input) -> String {
    let mut memo = std::collections::HashMap::new();
    let total: u64 = input
        .designs
        .iter()
        .map(|design| arrangements(design, &input.patterns, &mut memo))
        .sum();

    total.to_string()
}

#[test]
fn test_day19() {
    let input = parse(
        "r, wr, b, g, bwu, rb, gb, br

         brwrr
         bggr
         gbbr
         rrbgbr
         ubwu
         bwurrg
         brgr
         bbrgwb",
    );
    assert_eq!(part1(&input), "6");
    assert_eq!(part2(&input), "16");
}
