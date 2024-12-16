use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn left(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
    }
}

fn right(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn delta(dir: Direction) -> (i32, i32) {
    match dir {
        Direction::North => (-1, 0),
        Direction::East => (0, 1),
        Direction::South => (1, 0),
        Direction::West => (0, -1),
    }
}

pub struct Maze {
    grid: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    cost: i32,
    pos: (usize, usize),
    dir: Direction,
}

impl State {
    fn new(cost: i32, pos: (usize, usize), dir: Direction) -> Self {
        Self { cost, pos, dir }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse(input: &str) -> Maze {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let (start, end) = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, &cell)| match cell {
                    'S' => Some((i, j, true)),
                    'E' => Some((i, j, false)),
                    _ => None,
                })
        })
        .fold(((0, 0), (0, 0)), |acc, (i, j, is_start)| match is_start {
            true => ((i, j), acc.1),
            false => (acc.0, (i, j)),
        });

    Maze { grid, start, end }
}

fn explore_paths(
    maze: &Maze,
    start: (usize, usize),
    dirs: &[Direction],
    reverse: bool,
) -> HashMap<((usize, usize), Direction), i32> {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    let mut costs = HashMap::new();

    dirs.iter().for_each(|&dir| {
        heap.push(State::new(0, start, dir));
    });

    while let Some(State { cost, pos, dir }) = heap.pop() {
        if !seen.insert((pos, dir)) {
            continue;
        }

        costs.insert((pos, dir), cost);

        let (dx, dy) = delta(dir);
        let nx = pos.0 as i32 + (if reverse { -dx } else { dx });
        let dx = pos.1 as i32 + (if reverse { -dy } else { dy });

        if nx >= 0 && nx < maze.grid.len() as i32 && dx >= 0 && dx < maze.grid[0].len() as i32 {
            let new_pos = (nx as usize, dx as usize);
            if maze.grid[new_pos.0][new_pos.1] != '#' {
                heap.push(State::new(cost + 1, new_pos, dir));
            }
        }

        heap.push(State::new(cost + 1000, pos, left(dir)));
        heap.push(State::new(cost + 1000, pos, right(dir)));
    }

    costs
}

fn shortest_path(maze: &Maze) -> i32 {
    explore_paths(maze, maze.start, &[Direction::East], false)
        .iter()
        .filter(|((pos, _), _)| *pos == maze.end)
        .map(|(_, &cost)| cost)
        .min()
        .unwrap()
}

fn optimal_tiles(maze: &Maze) -> HashSet<(usize, usize)> {
    let all_dirs = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    let f_costs = explore_paths(maze, maze.start, &[Direction::East], false);
    let b_costs = explore_paths(maze, maze.end, &all_dirs, true);
    let shortest = shortest_path(maze);

    (0..maze.grid.len())
        .cartesian_product(0..maze.grid[0].len())
        .filter(|&(r, c)| {
            all_dirs.iter().any(|&dir| {
                match (f_costs.get(&((r, c), dir)), b_costs.get(&((r, c), dir))) {
                    (Some(&forw), Some(&back)) => forw + back == shortest,
                    _ => false,
                }
            })
        })
        .collect()
}

pub fn part1(input: &Maze) -> String {
    shortest_path(input).to_string()
}

pub fn part2(input: &Maze) -> String {
    optimal_tiles(input).len().to_string()
}

#[test]
fn test_day16() {
    let input = parse(
        "###############
         #.......#....E#
         #.#.###.#.###.#
         #.....#.#...#.#
         #.###.#####.#.#
         #.#.#.......#.#
         #.#.#####.###.#
         #...........#.#
         ###.#.#####.#.#
         #...#.....#.#.#
         #.#.#.###.#.#.#
         #.....#...#.#.#
         #.###.#.#.#.#.#
         #S..#.....#...#
         ###############",
    );
    assert_eq!(part1(&input), "7036");
    assert_eq!(part2(&input), "45");
}
