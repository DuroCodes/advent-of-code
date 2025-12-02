use rayon::prelude::*;
use std::ops::RangeInclusive;

pub fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect()
}

fn has_repetition(id_str: &str, k: usize) -> bool {
    let len = id_str.len();
    len % k == 0 && {
        let part_len = len / k;
        let first_part = &id_str[0..part_len];
        (1..k).all(|i| &id_str[i * part_len..(i + 1) * part_len] == first_part)
    }
}

fn invalid_id(id: u64, min_repetitions: usize) -> bool {
    let id_str = id.to_string();
    (min_repetitions..=id_str.len()).any(|k| has_repetition(&id_str, k))
}

pub fn part1(input: &[RangeInclusive<u64>]) -> String {
    input
        .par_iter()
        .flat_map(|range| range.clone())
        .filter(|&id| has_repetition(&id.to_string(), 2))
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: &[RangeInclusive<u64>]) -> String {
    input
        .par_iter()
        .flat_map(|range| range.clone())
        .filter(|&id| invalid_id(id, 2))
        .sum::<u64>()
        .to_string()
}

#[test]
fn test_day02() {
    let input = parse("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124");
    assert_eq!(part1(&input), "1227775554");
    assert_eq!(part2(&input), "4174379265");
}
