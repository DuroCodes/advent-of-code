use itertools::{iproduct, Itertools};
use rayon::prelude::*;
use std::collections::HashSet;

pub struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

pub fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let (start, end) = (line.find('[').unwrap(), line.find(']').unwrap());
            let lights = line[start + 1..end].chars().map(|c| c == '#').collect();

            let rest = &line[end + 1..];
            let buttons = rest
                .split('{')
                .next()
                .unwrap_or(rest)
                .split('(')
                .filter_map(|p| p.find(')').map(|e| &p[..e]))
                .filter(|s| !s.is_empty())
                .map(|s| s.split(',').filter_map(|n| n.trim().parse().ok()).collect())
                .collect();

            let joltage = rest
                .find('{')
                .map(|s| {
                    rest[s + 1..rest.find('}').unwrap_or(rest.len())]
                        .split(',')
                        .filter_map(|s| s.trim().parse().ok())
                        .collect()
                })
                .unwrap_or_default();

            Machine {
                lights,
                buttons,
                joltage,
            }
        })
        .collect()
}

fn min_presses_lights(target: &[bool], buttons: &[Vec<usize>]) -> usize {
    (0u64..(1u64 << buttons.len()))
        .filter(|&mask| {
            buttons
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1u64 << i) != 0)
                .flat_map(|(_, btn)| btn.iter().copied())
                .filter(|&idx| idx < target.len())
                .fold(vec![false; target.len()], |mut s, i| {
                    s[i] ^= true;
                    s
                })
                == target
        })
        .map(|mask| mask.count_ones() as usize)
        .min()
        .unwrap_or(usize::MAX)
}

pub fn part1(machines: &[Machine]) -> String {
    machines
        .iter()
        .map(|m| min_presses_lights(&m.lights, &m.buttons))
        .sum::<usize>()
        .to_string()
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a.abs().max(1),
        _ => gcd(b, a % b),
    }
}

fn gcd_row(row: &[i64]) -> i64 {
    row.iter().copied().filter(|&x| x != 0).fold(0, gcd).max(1)
}

fn build_matrix(nc: usize, nb: usize, targets: &[usize], buttons: &[Vec<usize>]) -> Vec<Vec<i64>> {
    (0..nc)
        .map(|i| {
            (0..=nb)
                .map(|j| match j {
                    _ if j == nb => targets[i] as i64,
                    _ if buttons[j].contains(&i) => 1,
                    _ => 0,
                })
                .collect()
        })
        .collect()
}

// https://www.wikiwand.com/en/articles/Gaussian_elimination
fn gaussian_eliminate(matrix: &mut [Vec<i64>], nb: usize) -> Vec<(usize, usize)> {
    let nc = matrix.len();
    (0..nb)
        .fold((Vec::new(), 0), |(mut pivots, mut row), col| {
            (row..nc).find(|&r| matrix[r][col] != 0).map(|pr| {
                matrix.swap(row, pr);
                pivots.push((row, col));

                let pval = matrix[row][col];
                let elim: Vec<_> = (0..nc)
                    .filter(|&r| r != row && matrix[r][col] != 0)
                    .map(|r| (r, matrix[r][col]))
                    .collect();

                iproduct!(&elim, 0..=nb).for_each(|(&(r, fac), c)| {
                    matrix[r][c] = matrix[r][c] * pval - matrix[row][c] * fac;
                });

                elim.iter().for_each(|&(r, _)| {
                    let g = gcd_row(&matrix[r]);
                    (g > 1).then(|| matrix[r].iter_mut().for_each(|x| *x /= g));
                });

                row += 1;
            });
            (pivots, row)
        })
        .0
}

fn solve_x(
    free_vals: &[i64],
    free_cols: &[usize],
    matrix: &[Vec<i64>],
    pivots: &[(usize, usize)],
    nb: usize,
) -> Option<Vec<i64>> {
    let x = free_cols
        .iter()
        .zip(free_vals)
        .fold(vec![0i64; nb], |mut x, (&col, &val)| {
            x[col] = val;
            x
        });

    pivots
        .iter()
        .rev()
        .try_fold(x, |mut x, &(row, col)| {
            let pivot = matrix[row][col];
            let rhs = matrix[row][nb]
                - (0..nb)
                    .filter(|&c| c != col)
                    .map(|c| matrix[row][c] * x[c])
                    .sum::<i64>();
            (pivot != 0 && rhs % pivot == 0).then(|| {
                x[col] = rhs / pivot;
                x
            })
        })
        .filter(|x| x.iter().all(|&v| v >= 0))
}

fn search_min(
    bounds: &[usize],
    free_cols: &[usize],
    matrix: &[Vec<i64>],
    pivots: &[(usize, usize)],
    nb: usize,
) -> usize {
    bounds
        .iter()
        .map(|&b| 0..=b)
        .multi_cartesian_product()
        .filter_map(|vals| {
            let vals: Vec<i64> = vals.into_iter().map(|v| v as i64).collect();
            solve_x(&vals, free_cols, matrix, pivots, nb)
        })
        .map(|x| x.iter().sum::<i64>() as usize)
        .min()
        .unwrap_or(usize::MAX)
}

fn min_presses_joltage(targets: &[usize], buttons: &[Vec<usize>]) -> usize {
    let (nb, nc) = (buttons.len(), targets.len());

    (nb == 0)
        .then(|| {
            targets
                .iter()
                .all(|&t| t == 0)
                .then_some(0)
                .unwrap_or(usize::MAX)
        })
        .unwrap_or_else(|| {
            let mut matrix = build_matrix(nc, nb, targets, buttons);
            let pivots = gaussian_eliminate(&mut matrix, nb);

            matrix[pivots.len()..]
                .iter()
                .any(|row| row[nb] != 0)
                .then_some(usize::MAX)
                .unwrap_or_else(|| {
                    let basic: HashSet<_> = pivots.iter().map(|&(_, c)| c).collect();
                    let free_cols: Vec<_> = (0..nb).filter(|c| !basic.contains(c)).collect();

                    let bounds: Vec<_> = free_cols
                        .iter()
                        .map(|&j| {
                            buttons[j]
                                .iter()
                                .filter(|&&i| i < nc)
                                .map(|&i| targets[i])
                                .min()
                                .unwrap_or(0)
                        })
                        .collect();

                    search_min(&bounds, &free_cols, &matrix, &pivots, nb)
                })
        })
}

pub fn part2(machines: &[Machine]) -> String {
    machines
        .par_iter()
        .map(|m| min_presses_joltage(&m.joltage, &m.buttons))
        .sum::<usize>()
        .to_string()
}

#[test]
fn test_day10() {
    let input = parse(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    );
    assert_eq!(part1(&input), "7");
    assert_eq!(part2(&input), "33");
}
