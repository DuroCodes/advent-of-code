pub fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let (dir, dist) = line.split_at(1);
            let dist: i32 = dist.parse().unwrap();
            match dir {
                "L" => -dist,
                "R" => dist,
                _ => panic!("Invalid direction: {dir}"),
            }
        })
        .collect()
}

fn normalize(pos: i32) -> i32 {
    pos.rem_euclid(100)
}

pub fn part1(input: &[i32]) -> String {
    input
        .iter()
        .fold((50i32, 0u32), |(pos, count), &rotation| {
            let new_pos = normalize(pos + rotation);
            (new_pos, count + (new_pos == 0) as u32)
        })
        .1
        .to_string()
}

pub fn part2(input: &[i32]) -> String {
    input
        .iter()
        .fold((50i32, 0u32), |(start_pos, count), &rotation| {
            let zeros = (1..=rotation.abs() as u32)
                .map(|click| normalize(start_pos + rotation.signum() * click as i32))
                .filter(|&pos| pos == 0)
                .count() as u32;
            (normalize(start_pos + rotation), count + zeros)
        })
        .1
        .to_string()
}

#[test]
fn test_day01() {
    let input = parse("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82");
    assert_eq!(part1(&input), "3");
    assert_eq!(part2(&input), "6");
}
