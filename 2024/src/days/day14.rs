use itertools::Itertools;
use regex::Regex;

type Point = (i32, i32);
pub type Robot = (Point, Point);

pub fn parse(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                (caps[1].parse().unwrap(), caps[2].parse().unwrap()),
                (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
            )
        })
        .collect()
}

fn simulate(robots: &[Robot], width: i32, height: i32) -> Vec<Robot> {
    robots
        .iter()
        .map(move |&(pos, vel)| {
            (
                (
                    (pos.0 + vel.0).rem_euclid(width),
                    (pos.1 + vel.1).rem_euclid(height),
                ),
                vel,
            )
        })
        .collect()
}

fn robots_in_quads(robots: &[Robot], width: i32, height: i32) -> (usize, usize, usize, usize) {
    let mid_x = width / 2;
    let mid_y = height / 2;

    robots
        .iter()
        .filter(|r| r.0 .0 != mid_x && r.0 .1 != mid_y)
        .fold((0, 0, 0, 0), |counts, (pos, _)| {
            match (pos.0 < mid_x, pos.1 < mid_y) {
                (true, true) => (counts.0 + 1, counts.1, counts.2, counts.3),
                (false, true) => (counts.0, counts.1 + 1, counts.2, counts.3),
                (true, false) => (counts.0, counts.1, counts.2 + 1, counts.3),
                (false, false) => (counts.0, counts.1, counts.2, counts.3 + 1),
            }
        })
}

fn robot_density(robots: &[Robot]) -> f64 {
    let (sum, count) = robots
        .iter()
        .map(|(pos, _)| pos)
        .tuple_combinations()
        .map(|(a, b)| {
            let dx = (a.0 - b.0).abs() as f64;
            let dy = (a.1 - b.1).abs() as f64;
            (dx * dx + dy * dy).sqrt()
        })
        .fold((0.0, 0), |(sum, count), dist| (sum + dist, count + 1));

    sum / count as f64 // assume count > 0
}

pub fn part1(input: &[Robot]) -> String {
    let final_state = (0..100).fold(input.to_vec(), |robots, _| simulate(&robots, 101, 103));

    let (a, b, c, d) = robots_in_quads(&final_state, 101, 103);
    (a * b * c * d).to_string()
}

pub fn part2(input: &[Robot]) -> String {
    (0..20000)
        .scan(input.to_vec(), |state, t| {
            let density = robot_density(state);
            *state = simulate(state, 101, 103);
            Some((t, density))
        })
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(t, _)| t.to_string())
        .unwrap()
}

#[test]
fn test_day14() {
    let input: Vec<((i32, i32), (i32, i32))> = parse(
        "p=0,4 v=3,-3
         p=6,3 v=-1,-3
         p=10,3 v=-1,2
         p=2,0 v=2,-1
         p=0,0 v=1,3
         p=3,0 v=-2,-2
         p=7,6 v=-1,-3
         p=3,0 v=-1,-2
         p=9,3 v=2,3
         p=7,3 v=-1,2
         p=2,4 v=2,-3
         p=9,5 v=-3,-3",
    );
    assert_eq!(part1(&input), "21");
    assert_eq!(part2(&input), "5253");
}
