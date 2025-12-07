use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .filter(|row: &Vec<char>| !row.is_empty())
        .collect()
}

fn find_start(grid: &[Vec<char>]) -> usize {
    grid.iter()
        .find_map(|row| row.iter().position(|&c| c == 'S'))
        .unwrap()
}

fn next_cols(ch: char, col: usize, max_col: usize) -> Vec<usize> {
    match ch {
        '.' | 'S' => vec![col],
        '^' => [col.checked_sub(1), (col + 1 < max_col).then_some(col + 1)]
            .into_iter()
            .flatten()
            .collect(),
        _ => vec![],
    }
}

pub fn part1(grid: &[Vec<char>]) -> String {
    let start = find_start(grid);

    (0..grid.len() - 1)
        .fold((HashSet::from([start]), 0), |(beams, splits), row| {
            let new_splits = beams.iter().filter(|&&c| grid[row + 1][c] == '^').count();
            let next = beams
                .into_iter()
                .flat_map(|c| next_cols(grid[row + 1][c], c, grid[0].len()))
                .collect();

            (next, splits + new_splits)
        })
        .1
        .to_string()
}

pub fn part2(grid: &[Vec<char>]) -> String {
    let start = find_start(grid);

    (0..grid.len() - 1)
        .fold(HashMap::from([(start, 1u64)]), |counts, row| {
            counts
                .into_iter()
                .flat_map(|(c, n)| {
                    next_cols(grid[row + 1][c], c, grid[0].len())
                        .into_iter()
                        .map(move |c| (c, n))
                })
                .into_grouping_map()
                .sum()
        })
        .values()
        .sum::<u64>()
        .to_string()
}

#[test]
fn test_day07() {
    let input = parse(concat!(
        ".......S.......\n",
        "...............\n",
        ".......^.......\n",
        "...............\n",
        "......^.^......\n",
        "...............\n",
        ".....^.^.^.....\n",
        "...............\n",
        "....^.^...^....\n",
        "...............\n",
        "...^.^...^.^...\n",
        "...............\n",
        "..^...^.....^..\n",
        "...............\n",
        ".^.^.^.^.^...^.\n",
        "..............."
    ));
    assert_eq!(part1(&input), "21");
    assert_eq!(part2(&input), "40");
}
