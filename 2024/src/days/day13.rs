use regex::Regex;

type Point = (i64, i64);
pub type Machine = (Point, Point, Point);

pub fn parse(input: &str) -> Vec<Machine> {
    let button_re = Regex::new(r"Button [AB]: X([+-]\d+), Y([+-]\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    input
        .split("\n\n")
        .filter(|group| !group.is_empty())
        .map(|group| {
            let lines = group.lines().collect::<Vec<_>>();
            let parse_coords = |caps: regex::Captures<'_>| {
                (
                    caps[1].parse::<i64>().unwrap(),
                    caps[2].parse::<i64>().unwrap(),
                )
            };

            (
                button_re.captures(lines[0]).map(parse_coords).unwrap(),
                button_re.captures(lines[1]).map(parse_coords).unwrap(),
                prize_re.captures(lines[2]).map(parse_coords).unwrap(),
            )
        })
        .collect()
}

fn solve_machine(((a_x, a_y), (b_x, b_y), (p_x, p_y)): &Machine, offset: i64) -> Option<i64> {
    let (target_x, target_y) = (p_x + offset, p_y + offset);

    let det = a_x * b_y - a_y * b_x;
    if det == 0 {
        return None;
    }

    // cramers rule
    let (press_a, press_b) = (
        (target_x * b_y - target_y * b_x) as f64 / det as f64,
        (a_x * target_y - a_y * target_x) as f64 / det as f64,
    );

    let valid_solution =
        |a: f64, b: f64| a >= 0.0 && b >= 0.0 && a.fract().abs() < 1e-10 && b.fract().abs() < 1e-10;

    valid_solution(press_a, press_b)
        .then(|| {
            let (a, b) = (press_a.round() as i64, press_b.round() as i64);
            ((a * a_x + b * b_x == target_x) && (a * a_y + b * b_y == target_y))
                .then_some(a * 3 + b)
        })
        .flatten()
}

pub fn part1(input: &Vec<Machine>) -> String {
    input
        .iter()
        .filter_map(|m| solve_machine(m, 0))
        .sum::<i64>()
        .to_string()
}

pub fn part2(input: &Vec<Machine>) -> String {
    input
        .iter()
        .filter_map(|m| solve_machine(m, 10_000_000_000_000))
        .sum::<i64>()
        .to_string()
}

#[test]
fn test_day13() {
    let input = parse(
        "Button A: X+94, Y+34
         Button B: X+22, Y+67
         Prize: X=8400, Y=5400

         Button A: X+26, Y+66
         Button B: X+67, Y+21
         Prize: X=12748, Y=12176

         Button A: X+17, Y+86
         Button B: X+84, Y+37
         Prize: X=7870, Y=6450

         Button A: X+69, Y+23
         Button B: X+27, Y+71
         Prize: X=18641, Y=10279",
    );
    assert_eq!(part1(&input), "480");
    assert_eq!(part2(&input), "875318608908");
}
