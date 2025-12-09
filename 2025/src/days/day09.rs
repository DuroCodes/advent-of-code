use itertools::{iproduct, Itertools};
use std::collections::HashMap;

type Point = (i64, i64);
type PrefixSum = (Vec<Vec<i64>>, HashMap<i64, usize>, HashMap<i64, usize>);

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

fn area((x1, y1): &Point, (x2, y2): &Point) -> i64 {
    ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1)
}

pub fn part1(points: &[Point]) -> String {
    points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| area(a, b))
        .max()
        .unwrap_or(0)
        .to_string()
}

fn compressed_coords(points: &[Point]) -> (Vec<i64>, Vec<i64>) {
    let unique = |f: fn(&Point) -> i64| points.iter().map(f).sorted_unstable().dedup().collect();
    (unique(|p| p.0), unique(|p| p.1))
}

fn vertical_edges(points: &[Point]) -> HashMap<i64, Vec<(i64, i64)>> {
    points
        .iter()
        .circular_tuple_windows()
        .filter(|((x1, _), (x2, _))| x1 == x2)
        .map(|((x, y1), (_, y2))| (*x, (*y1.min(y2), *y1.max(y2))))
        .into_group_map()
}

fn inside_polygon(x: i64, y: i64, edges: &HashMap<i64, Vec<(i64, i64)>>) -> bool {
    edges
        .iter()
        .filter(|(&ex, _)| ex < x)
        .flat_map(|(_, segs)| segs)
        .filter(|(y1, y2)| y > *y1 && y <= *y2)
        .count()
        % 2
        == 1
}

fn inside_grid(points: &[Point], xs: &[i64], ys: &[i64]) -> Vec<Vec<bool>> {
    let edges = vertical_edges(points);
    let mid = |s: &[i64], i| (s[i] + s[i + 1]) / 2;

    (0..xs.len() - 1)
        .map(|i| {
            (0..ys.len() - 1)
                .map(|j| inside_polygon(mid(xs, i), mid(ys, j), &edges))
                .collect()
        })
        .collect()
}

fn index_map(slice: &[i64]) -> HashMap<i64, usize> {
    slice
        .iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect()
}

fn prefix_sum(inside: &[Vec<bool>], xs: &[i64], ys: &[i64]) -> PrefixSum {
    let (nx, ny) = (inside.len(), inside.first().map_or(0, Vec::len));

    // i hate mutability but it's the cleanest way to do this
    let mut prefix = vec![vec![0i64; ny + 1]; nx + 1];
    for (i, j) in iproduct!(0..nx, 0..ny) {
        let val = i64::from(inside[i][j]);
        prefix[i + 1][j + 1] = val + prefix[i][j + 1] + prefix[i + 1][j] - prefix[i][j];
    }

    (prefix, index_map(xs), index_map(ys))
}

fn valid_rect((prefix, xi, yi): &PrefixSum, a: &Point, b: &Point) -> bool {
    let (i1, i2, j1, j2) = (
        xi[&a.0.min(b.0)],
        xi[&a.0.max(b.0)],
        yi[&a.1.min(b.1)],
        yi[&a.1.max(b.1)],
    );

    prefix[i2][j2] - prefix[i1][j2] - prefix[i2][j1] + prefix[i1][j1]
        == ((i2 - i1) * (j2 - j1)) as i64
}

pub fn part2(points: &[Point]) -> String {
    let (xs, ys) = compressed_coords(points);
    let prefix = prefix_sum(&inside_grid(points, &xs, &ys), &xs, &ys);

    points
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| valid_rect(&prefix, a, b))
        .map(|(a, b)| area(a, b))
        .max()
        .unwrap_or(0)
        .to_string()
}

#[test]
fn test_day09() {
    let input = parse("7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3");
    assert_eq!(part1(&input), "50");
    assert_eq!(part2(&input), "24");
}
