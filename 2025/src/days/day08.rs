use itertools::Itertools;
use std::collections::HashSet;

type Point = (i64, i64, i64);

pub fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter_map(|line| {
            line.split(',')
                .filter_map(|n| n.trim().parse().ok())
                .collect_tuple()
        })
        .collect()
}

fn square_distance(a: &Point, b: &Point) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

fn sorted_pairs(points: &[Point]) -> Vec<(i64, usize, usize)> {
    (0..points.len())
        .tuple_combinations()
        .map(|(i, j)| (square_distance(&points[i], &points[j]), i, j))
        .sorted_unstable()
        .collect()
}

fn join(mut comps: Vec<HashSet<usize>>, i: usize, j: usize) -> Vec<HashSet<usize>> {
    let (ci, cj) = (
        comps.iter().position(|c| c.contains(&i)).unwrap(),
        comps.iter().position(|c| c.contains(&j)).unwrap(),
    );

    if ci != cj {
        let merged = comps[ci].union(&comps[cj]).copied().collect();
        comps.retain(|c| !c.contains(&i) && !c.contains(&j));
        comps.push(merged);
    }

    comps
}

fn connect(points: &[Point], n: usize) -> Vec<HashSet<usize>> {
    let init = (0..points.len()).map(|i| HashSet::from([i])).collect();

    sorted_pairs(points)
        .into_iter()
        .take(n)
        .fold(init, |comps, (_, i, j)| join(comps, i, j))
}

pub fn part1(points: &[Point]) -> String {
    connect(points, 1000)
        .iter()
        .map(|c| c.len())
        .sorted_unstable()
        .rev()
        .take(3)
        .product::<usize>()
        .to_string()
}

pub fn part2(points: &[Point]) -> String {
    let init = (0..points.len())
        .map(|i| HashSet::from([i]))
        .collect::<Vec<_>>();

    sorted_pairs(points)
        .into_iter()
        .scan(init, |comps, (_, i, j)| {
            *comps = join(comps.clone(), i, j);
            Some((comps.len() == 1, i, j))
        })
        .find(|(done, _, _)| *done)
        .map(|(_, i, j)| points[i].0 * points[j].0)
        .unwrap()
        .to_string()
}

#[test]
fn test_day08() {
    let input = parse(
        "162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689",
    );
    assert_eq!(part1(&input), "20"); // this differs from the actual test given, since it was only for 10 connections
    assert_eq!(part2(&input), "25272");
}
