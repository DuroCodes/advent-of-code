use itertools::Itertools;

pub type Input = (Vec<(u64, u64)>, Vec<u64>);

pub fn parse(input: &str) -> Input {
    let (ranges_str, ids_str) = input.trim().split_once("\n\n").unwrap();

    let ranges = ranges_str
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let ids = ids_str.lines().map(|line| line.parse().unwrap()).collect();

    (ranges, ids)
}

pub fn part1((ranges, ids): &Input) -> String {
    ids.iter()
        .filter(|&&id| ranges.iter().any(|&(start, end)| id >= start && id <= end))
        .count()
        .to_string()
}

pub fn part2((ranges, _): &Input) -> String {
    ranges
        .iter()
        .sorted_by_key(|&(start, _)| start)
        .fold(Vec::<(u64, u64)>::new(), |mut acc, &(start, end)| {
            match acc.last_mut() {
                Some((_, last_end)) if start <= *last_end + 1 => {
                    *last_end = (*last_end).max(end);
                }
                _ => acc.push((start, end)),
            }
            acc
        })
        .iter()
        .map(|&(start, end)| end - start + 1)
        .sum::<u64>()
        .to_string()
}

#[test]
fn test_day05() {
    // this is inlined with \n because i don't want to deal with whitespace in the split_once("\n\n")
    let input = parse("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32");
    assert_eq!(part1(&input), "3");
    assert_eq!(part2(&input), "14");
}
