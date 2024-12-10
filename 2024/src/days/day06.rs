use rayon::prelude::*;
use std::collections::HashSet;

type Grid = Vec<Vec<char>>;
type Point = (usize, usize);

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub struct Guard {
    pos: Point,
    direction: Direction,
}

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn step(&self, x: usize, y: usize, max_x: usize, max_y: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up if y > 0 => Some((x, y - 1)),
            Direction::Right if x < max_x => Some((x + 1, y)),
            Direction::Down if y < max_y => Some((x, y + 1)),
            Direction::Left if x > 0 => Some((x - 1, y)),
            _ => None,
        }
    }
}

impl Guard {
    fn new(x: usize, y: usize) -> Self {
        Self {
            pos: (x, y),
            direction: Direction::Up,
        }
    }

    fn rotate(&self) -> Self {
        Self {
            direction: self.direction.rotate(),
            ..*self
        }
    }

    fn step(&self, pos: Point) -> Self {
        Self { pos, ..*self }
    }
}

pub fn parse(input: &str) -> (Grid, Guard) {
    let grid: Grid = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let (x, y) = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &c)| (x, y, c)))
        .find(|&(_, _, c)| c == '^')
        .map(|(x, y, _)| (x, y))
        .unwrap();

    (grid, Guard::new(x, y))
}

fn next_state(grid: &Grid, guard: Guard) -> Option<Guard> {
    let (max_x, max_y) = (grid[0].len() - 1, grid.len() - 1);
    let (x, y) = guard.pos;

    guard
        .direction
        .step(x, y, max_x, max_y)
        .map(|new_pos| match grid[new_pos.1][new_pos.0] {
            '#' => guard.rotate(),
            _ => guard.step(new_pos),
        })
}

fn walk_path(grid: &Grid, start: Guard, obstruction: Option<Point>) -> HashSet<Point> {
    let mut seen = HashSet::new();
    seen.insert(start);

    std::iter::successors(Some(start), |&current| {
        next_state(grid, current).and_then(|next| match (obstruction, next.pos) {
            (Some(pos), next_pos) if pos == next_pos => Some(current.rotate()),
            _ => match seen.insert(next) {
                true => Some(next),
                false => None,
            },
        })
    })
    .map(|guard| guard.pos)
    .collect()
}

fn adjacent_positions(path: &HashSet<Point>, grid: &Grid) -> Vec<Point> {
    let (max_x, max_y) = (grid[0].len() as i32, grid.len() as i32);
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    path.iter()
        .flat_map(|&(x, y)| {
            directions.iter().filter_map(move |&(dx, dy)| {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                match nx >= 0 && nx < max_x && ny >= 0 && ny < max_y {
                    true => Some((nx as usize, ny as usize)),
                    false => None,
                }
            })
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn detect_loop(grid: &Grid, guard: Guard, obstacle: Point, max_steps: usize) -> bool {
    let edge = |p: Point| p.0 == 0 || p.0 == grid[0].len() - 1 || p.1 == 0 || p.1 == grid.len() - 1;
    let mut visited = HashSet::new();

    (0..max_steps)
        .try_fold(guard, |current, _| {
            if !visited.insert(current) {
                return Err(true);
            }

            match next_state(grid, current) {
                Some(next) if next.pos == obstacle => Ok(current.rotate()),
                Some(next) if edge(next.pos) => Err(false),
                Some(next) => Ok(next),
                None => Err(false),
            }
        })
        .err()
        == Some(true)
}

pub fn part1(input: &(Grid, Guard)) -> String {
    let (grid, guard) = input;
    walk_path(grid, *guard, None).len().to_string()
}

pub fn part2(input: &(Grid, Guard)) -> String {
    let (grid, guard) = input;

    let initial_path = walk_path(grid, *guard, None);
    let max_steps = grid.len() * grid[0].len() * 4;

    adjacent_positions(&initial_path, grid)
        .into_iter()
        .filter(|&pos| pos != guard.pos)
        .collect::<Vec<_>>()
        .into_par_iter()
        .filter(|&pos| detect_loop(grid, *guard, pos, max_steps))
        .count()
        .to_string()
}

#[test]
fn test_day06() {
    let input = parse(
        "....#.....
         .........#
         ..........
         ..#.......
         .......#..
         ..........
         .#..^.....
         ........#.
         #.........
         ......#...",
    );
    assert_eq!(part1(&input), "41");
    assert_eq!(part2(&input), "6");
}
