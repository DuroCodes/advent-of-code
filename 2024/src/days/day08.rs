use itertools::Itertools;
use std::collections::HashSet;

type Antenna = (i32, i32, char);
type Point = (i32, i32);

pub struct Grid {
    antennas: Vec<Antenna>,
    width: i32,
    height: i32,
}

pub fn parse(input: &str) -> Grid {
    let lines: Vec<_> = input.lines().map(str::trim).collect();

    Grid {
        antennas: lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .map(move |(x, c)| (x as i32, y as i32, c))
            })
            .collect(),
        height: lines.len() as i32,
        width: lines[0].len() as i32,
    }
}

fn freq_groups(antennas: &[Antenna]) -> Vec<Vec<Point>> {
    antennas
        .iter()
        .sorted_by_key(|&&(_, _, f)| f)
        .chunk_by(|&&(_, _, f)| f)
        .into_iter()
        .map(|(_, group)| group.map(|&(x, y, _)| (x, y)).collect::<Vec<_>>())
        .filter(|g| g.len() >= 2)
        .collect()
}

fn resonant((x1, y1): Point, (x2, y2): Point, (px, py): Point) -> bool {
    let d1: i32 = (px - x1).abs() + (py - y1).abs();
    let d2 = (px - x2).abs() + (py - y2).abs();
    (d1 == 2 * d2 || d2 == 2 * d1) && (px - x1) * (y2 - y1) == (x2 - x1) * (py - y1)
}

fn collinear((x1, y1): Point, (x2, y2): Point, (px, py): Point) -> bool {
    (px - x1) * (y2 - y1) == (x2 - x1) * (py - y1)
}

fn solve(
    antennas: &[Antenna],
    grid: &Grid,
    check_fn: fn(Point, Point, Point) -> bool,
    include_antennas: bool,
) -> HashSet<Point> {
    freq_groups(antennas)
        .into_iter()
        .flat_map(|group| {
            let points = (0..grid.height)
                .cartesian_product(0..grid.width)
                .map(|(y, x)| (x, y))
                .filter(|&p| {
                    group
                        .iter()
                        .tuple_combinations()
                        .any(|(&a1, &a2)| check_fn(a1, a2, p))
                });

            match include_antennas {
                true => points.chain(group.iter().copied()).collect::<HashSet<_>>(),
                false => points.collect(),
            }
        })
        .collect()
}

pub fn part1(input: &Grid) -> String {
    solve(&input.antennas, input, resonant, false)
        .len()
        .to_string()
}

pub fn part2(input: &Grid) -> String {
    solve(&input.antennas, input, collinear, true)
        .len()
        .to_string()
}

#[test]
fn test_day08() {
    let input = parse(
        "............
         ........0...
         .....0......
         .......0....
         ....0.......
         ......A.....
         ............
         ............
         ........A...
         .........A..
         ............
         ............",
    );
    assert_eq!(part1(&input), "14");
    assert_eq!(part2(&input), "34");
}
