use itertools::iproduct;
use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn count_neighbors(grid: &[Vec<char>], i: usize, j: usize) -> usize {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    iproduct!(-1..=1, -1..=1)
        .filter(|&(di, dj)| di != 0 || dj != 0)
        .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
        .filter(|&(ni, nj)| ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32)
        .filter(|&(ni, nj)| grid[ni as usize][nj as usize] == '@')
        .count()
}

fn find_isolated(grid: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    iproduct!(0..rows, 0..cols)
        .filter(move |&(i, j)| grid[i][j] == '@')
        .filter(move |&(i, j)| count_neighbors(grid, i, j) < 4)
        .collect()
}

pub fn part1(input: &[Vec<char>]) -> String {
    find_isolated(input).len().to_string()
}

fn remove_round(grid: &[Vec<char>]) -> (Vec<Vec<char>>, usize) {
    let to_remove = find_isolated(grid);

    let new_grid = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                // i hate if/else
                .map(|(j, &cell)| to_remove.contains(&(i, j)).then_some('.').unwrap_or(cell))
                .collect()
        })
        .collect::<Vec<_>>();

    (new_grid, to_remove.len())
}

fn remove_all(grid: Vec<Vec<char>>, total_removed: usize) -> usize {
    let (grid, removed) = remove_round(&grid);

    match removed {
        0 => total_removed,
        _ => remove_all(grid, total_removed + removed),
    }
}

pub fn part2(input: &[Vec<char>]) -> String {
    remove_all(input.to_vec(), 0).to_string()
}

#[test]
fn test_day04() {
    let input = parse(
        "..@@.@@@@.
         @@@.@.@.@@
         @@@@@.@.@@
         @.@@@@..@.
         @@.@@@@.@@
         .@@@@@@@.@
         .@.@.@.@@@
         @.@@@.@@@@
         .@@@@@@@@.
         @.@.@@@.@.",
    );
    assert_eq!(part1(&input), "13");
    assert_eq!(part2(&input), "43");
}
