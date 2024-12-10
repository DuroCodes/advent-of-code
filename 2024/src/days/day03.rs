use itertools::Itertools;
use regex::Regex;

pub enum Instruction {
    Mul(u32, u32),
    Control(bool),
}

pub fn parse(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(?P<ctrl>do|don't)\(\)").unwrap();

    re.find_iter(input)
        .filter_map(|m| {
            let caps = re.captures(m.as_str())?;
            Some((
                m.start(),
                match caps.name("ctrl") {
                    Some(ctrl) => Instruction::Control(ctrl.as_str() == "do"),
                    None => Instruction::Mul(caps[1].parse().ok()?, caps[2].parse().ok()?),
                },
            ))
        })
        .sorted_by_key(|(pos, _)| *pos)
        .map(|(_, inst)| inst)
        .collect()
}

pub fn part1(input: &[Instruction]) -> String {
    input
        .iter()
        .fold(0, |sum, inst| match inst {
            Instruction::Mul(x, y) => sum + x * y,
            _ => sum,
        })
        .to_string()
}

pub fn part2(input: &[Instruction]) -> String {
    input
        .iter()
        .fold((true, 0), |(enabled, sum), inst| match inst {
            Instruction::Control(ctrl) => (*ctrl, sum),
            Instruction::Mul(x, y) if enabled => (enabled, sum + x * y),
            _ => (enabled, sum),
        })
        .1
        .to_string()
}

#[test]
fn test_day03() {
    let input = parse("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
    assert_eq!(part1(&input), "161");
    assert_eq!(part2(&input), "48");
}
