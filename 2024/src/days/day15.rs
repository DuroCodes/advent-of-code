use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

type Grid = Vec<Vec<char>>;

pub struct Puzzle {
    grid: Grid,
    instructions: String,
}

pub fn parse(input: &str) -> Puzzle {
    let trimmed = input.lines().map(str::trim).join("\n");
    let (grid, instructions) = trimmed.split_once("\n\n").unwrap();
    let grid = grid.lines().map(|line| line.chars().collect()).collect();
    let instructions = instructions.trim().to_string();

    Puzzle { grid, instructions }
}

fn expand_grid(grid: Grid) -> Grid {
    grid.into_iter()
        .map(|row| {
            row.into_iter()
                .flat_map(|c| match c {
                    '#' => vec!['#', '#'],
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    '@' => vec!['@', '.'],
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn start(grid: &Grid) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &c)| c == '@')
                .map(|(c, _)| (r, c))
        })
        .unwrap()
}

fn process(grid: &mut Grid, r: usize, c: usize, dr: i32, dc: i32) -> Option<(usize, usize)> {
    let (rr, cc) = ((r as i32 + dr) as usize, (c as i32 + dc) as usize);

    match grid[rr][cc] {
        '#' => None,
        '.' => Some((rr, cc)),
        '[' | ']' | 'O' => {
            let mut queue = VecDeque::from([(r, c)]);
            let mut seen = HashSet::new();
            let mut stop = false;

            while let Some((curr_r, curr_c)) = queue.pop_front() {
                if seen.contains(&(curr_r, curr_c)) {
                    continue;
                }
                seen.insert((curr_r, curr_c));

                let (n_r, n_c) = ((curr_r as i32 + dr) as usize, (curr_c as i32 + dc) as usize);
                match grid[n_r][n_c] {
                    '#' => {
                        stop = true;
                        break;
                    }
                    'O' | '[' | ']' => {
                        queue.push_back((n_r, n_c));
                        match grid[n_r][n_c] {
                            '[' => queue.push_back((n_r, n_c + 1)),
                            ']' => queue.push_back((n_r, n_c - 1)),
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }

            if stop {
                return None;
            }

            let mut blocks: Vec<_> = seen.iter().copied().collect();
            while !blocks.is_empty() {
                if let Some(idx) = blocks.iter().position(|&(rr, cc)| {
                    let (n_r, n_c) = ((rr as i32 + dr) as usize, (cc as i32 + dc) as usize);
                    !blocks.contains(&(n_r, n_c))
                }) {
                    let (rr, cc) = blocks.remove(idx);
                    let (n_r, n_c) = ((rr as i32 + dr) as usize, (cc as i32 + dc) as usize);
                    grid[n_r][n_c] = grid[rr][cc];
                    grid[rr][cc] = '.';
                }
            }

            Some(((r as i32 + dr) as usize, (c as i32 + dc) as usize))
        }
        _ => unreachable!(),
    }
}

fn solve(input: &Puzzle, expand: bool) -> String {
    let mut grid = match expand {
        true => expand_grid(input.grid.clone()),
        false => input.grid.clone(),
    };

    let (r, c) = start(&grid);
    grid[r][c] = '.';

    input
        .instructions
        .chars()
        .filter_map(|inst| match inst {
            '^' => Some((-1, 0)),
            '>' => Some((0, 1)),
            'v' => Some((1, 0)),
            '<' => Some((0, -1)),
            _ => None,
        })
        .fold((r, c), |(r, c), (dr, dc)| {
            process(&mut grid, r, c, dr, dc).unwrap_or((r, c))
        });

    let grid: &Grid = &grid;
    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter(|&(r, c)| matches!(grid[r][c], '[' | 'O'))
        .map(|(r, c)| (100 * r + c) as i32)
        .sum::<i32>()
        .to_string()
}

pub fn part1(input: &Puzzle) -> String {
    solve(input, false)
}

pub fn part2(input: &Puzzle) -> String {
    solve(input, true)
}

#[test]
fn test_day15() {
    let input = parse(
        "########
         #..O.O.#
         ##@.O..#
         #...O..#
         #.#.O..#
         #...O..#
         #......#
         ########

         <^^>>>vv<v>>v<<",
    );
    assert_eq!(part1(&input), "2028");
    assert_eq!(part2(&input), "1751");
}
