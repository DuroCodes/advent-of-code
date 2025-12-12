use rayon::prelude::*;
use std::collections::HashSet;

type Shape = Vec<(i32, i32)>;

pub struct Input {
    shapes: Vec<Vec<Shape>>,
    regions: Vec<(usize, usize, Vec<usize>)>,
}

fn normalize(shape: &Shape) -> Shape {
    let (min_r, min_c) = shape
        .iter()
        .fold((i32::MAX, i32::MAX), |(mr, mc), &(r, c)| {
            (mr.min(r), mc.min(c))
        });

    let mut norm: Shape = shape.iter().map(|(r, c)| (r - min_r, c - min_c)).collect();
    norm.sort();
    norm
}

fn all_orientations(shape: &Shape) -> Vec<Shape> {
    (0..4)
        .fold(
            (shape.clone(), HashSet::new(), Vec::new()),
            |(cur, mut seen, mut res), _| {
                let norm = normalize(&cur);
                seen.insert(norm.clone()).then(|| res.push(norm));
                let flip = normalize(&cur.iter().map(|&(r, c)| (r, -c)).collect());
                seen.insert(flip.clone()).then(|| res.push(flip));
                (cur.iter().map(|&(r, c)| (c, -r)).collect(), seen, res)
            },
        )
        .2
}

pub fn parse(input: &str) -> Input {
    let shapes = input
        .split("\n\n")
        .filter(|p| {
            p.lines()
                .next()
                .is_some_and(|l| l.contains(':') && !l.contains('x'))
        })
        .map(|p| {
            let cells: Shape = p
                .lines()
                .skip(1)
                .enumerate()
                .flat_map(|(r, l)| {
                    l.chars()
                        .enumerate()
                        .filter(|&(_, c)| c == '#')
                        .map(move |(c, _)| (r as i32, c as i32))
                })
                .collect();
            all_orientations(&cells)
        })
        .collect();

    let regions = input
        .lines()
        .filter_map(|line| {
            let (dims, counts) = line.split_once(':')?;
            let (w, h) = dims.split_once('x')?;
            Some((
                w.trim().parse().ok()?,
                h.trim().parse().ok()?,
                counts
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect(),
            ))
        })
        .collect();

    Input { shapes, regions }
}

fn can_place(
    grid: &[Vec<bool>],
    shape: &Shape,
    (pr, pc): (usize, usize),
    h: usize,
    w: usize,
) -> bool {
    shape.iter().all(|&(dr, dc)| {
        let (r, c) = (pr as i32 + dr, pc as i32 + dc);
        r >= 0 && c >= 0 && (r as usize) < h && (c as usize) < w && !grid[r as usize][c as usize]
    })
}

fn place(grid: &mut [Vec<bool>], shape: &Shape, (pr, pc): (usize, usize), val: bool) {
    shape
        .iter()
        .for_each(|&(dr, dc)| grid[(pr as i32 + dr) as usize][(pc as i32 + dc) as usize] = val);
}

fn remaining_cells(shapes: &[Vec<Shape>], counts: &[usize]) -> usize {
    shapes
        .iter()
        .zip(counts)
        .map(|(o, &c)| o.first().map_or(0, |s| s.len() * c))
        .sum()
}

fn solve(
    grid: &mut Vec<Vec<bool>>,
    shapes: &[Vec<Shape>],
    counts: &mut [usize],
    h: usize,
    w: usize,
    min_pos: usize,
) -> bool {
    let Some(idx) = counts.iter().position(|&c| c > 0) else {
        return true;
    };

    let (sr, sc) = (min_pos / w, min_pos % w);

    (sr..h).any(|r| {
        ((r > sr).then_some(0).unwrap_or(sc)..w).any(|c| {
            shapes[idx].iter().any(|shape| {
                can_place(grid, shape, (r, c), h, w) && {
                    place(grid, shape, (r, c), true);
                    counts[idx] -= 1;

                    let found = remaining_cells(shapes, counts)
                        <= grid.iter().flatten().filter(|&&x| !x).count()
                        && solve(
                            grid,
                            shapes,
                            counts,
                            h,
                            w,
                            (counts[idx] > 0).then_some(r * w + c).unwrap_or(0),
                        );

                    found.then_some(()).unwrap_or_else(|| {
                        counts[idx] += 1;
                        place(grid, shape, (r, c), false);
                    });

                    found
                }
            })
        })
    })
}

fn can_fit(shapes: &[Vec<Shape>], w: usize, h: usize, counts: &[usize]) -> bool {
    remaining_cells(shapes, counts) <= w * h
        && (counts.iter().sum::<usize>() == 0
            || solve(
                &mut vec![vec![false; w]; h],
                shapes,
                &mut counts.to_vec(),
                h,
                w,
                0,
            ))
}

pub fn part1(input: &Input) -> String {
    input
        .regions
        .par_iter()
        .filter(|(w, h, c)| can_fit(&input.shapes, *w, *h, c))
        .count()
        .to_string()
}

pub fn part2(_input: &Input) -> String {
    "0".to_string()
}

#[test]
fn test_day12() {
    let input = parse(concat!(
        "0:\n###\n##.\n##.\n\n",
        "1:\n###\n##.\n.##\n\n",
        "2:\n.##\n###\n##.\n\n",
        "3:\n##.\n###\n##.\n\n",
        "4:\n###\n#..\n###\n\n",
        "5:\n###\n.#.\n###\n\n",
        "4x4: 0 0 0 0 2 0\n",
        "12x5: 1 0 1 0 2 2\n",
        "12x5: 1 0 1 0 3 2",
    ));

    assert_eq!(part1(&input), "2");
    assert_eq!(part2(&input), "0");
}
