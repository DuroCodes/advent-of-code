use itertools::Itertools;

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn is_divider(grid: &[Vec<char>], col: usize) -> bool {
    grid.iter().all(|row| row[col] == ' ')
}

fn find_ranges(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let cols = grid[0].len();

    (0..=cols)
        .scan(None, |start, col| {
            (col < cols && !is_divider(grid, col))
                .then(|| start.get_or_insert(col))
                .and_then(|_| Some(None))
                .or_else(|| start.take().map(|start| Some((start, col))))
        })
        .flatten()
        .collect()
}

fn get_from_rows(grid: &[Vec<char>], start: usize, end: usize) -> Vec<u64> {
    let rows = grid.len() - 1;
    (0..rows)
        .flat_map(|row| {
            grid[row]
                .iter()
                .skip(start)
                .take(end - start)
                .copied()
                .chunk_by(|&ch| ch.is_ascii_digit())
                .into_iter()
                .filter_map(|(_, group)| group.collect::<String>().parse().ok())
                .collect::<Vec<u64>>()
        })
        .collect()
}

fn get_from_cols(grid: &[Vec<char>], start: usize, end: usize) -> Vec<u64> {
    (start..end)
        .filter_map(|col| {
            (0..grid.len() - 1)
                .filter_map(|row| grid.get(row).and_then(|row| row.get(col)).copied())
                .filter(|&ch| ch.is_ascii_digit())
                .collect::<String>()
                .parse()
                .ok()
        })
        .collect()
}

fn solve(grid: &[Vec<char>], get_numbers: fn(&[Vec<char>], usize, usize) -> Vec<u64>) -> u64 {
    find_ranges(grid)
        .into_iter()
        .filter_map(|(start, end)| {
            let numbers = &get_numbers(grid, start, end);

            grid.get(grid.len() - 1)?
                .iter()
                .skip(start)
                .take(end - start)
                .find(|&&ch| ch == '+' || ch == '*')
                .copied()
                .map(|op| match op {
                    '+' => numbers.iter().sum(),
                    '*' => numbers.iter().product(),
                    _ => 0,
                })
        })
        .sum::<u64>()
}

pub fn part1(input: &[Vec<char>]) -> String {
    solve(input, get_from_rows).to_string()
}

pub fn part2(input: &[Vec<char>]) -> String {
    solve(input, get_from_cols).to_string()
}

#[test]
fn test_day06() {
    let example = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
    let input = parse(example);
    assert_eq!(part1(&input), "4277556");
    assert_eq!(part2(&input), "3263827");
}
