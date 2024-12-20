use itertools::iproduct;
use std::collections::{HashSet, VecDeque};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

type Grid = Vec<Vec<char>>;
pub struct Maze {
    grid: Grid,
    start: (usize, usize),
    end: (usize, usize),
}

pub fn parse(input: &str) -> Maze {
    let grid: Grid = input
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

fn is_track(grid: &Grid, x: usize, y: usize) -> bool {
    grid[x][y] == '.' || grid[x][y] == 'S' || grid[x][y] == 'E'
}

fn bfs(grid: &Grid, start: (usize, usize), steps: Option<usize>, walls: bool) -> Vec<Vec<usize>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dist = vec![vec![usize::MAX; cols]; rows];
    let mut queue = VecDeque::new();

    dist[start.0][start.1] = 0;
    queue.push_back(start);

    while let Some((x, y)) = queue.pop_front() {
        let d = dist[x][y];
        if let Some(max) = steps {
            if d == max {
                continue;
            }
        }

        let neighbors: Vec<(usize, usize)> = DIRECTIONS
            .iter()
            .map(|&(dx, dy)| (x as i32 + dx, y as i32 + dy))
            .map(|(dx, dy)| (dx as usize, dy as usize))
            .filter(|&(nx, ny)| {
                nx < rows
                    && ny < cols
                    && (!walls || is_track(grid, nx, ny))
                    && dist[nx][ny] > dist[x][y] + 1
            })
            .collect();

        for (nx, ny) in neighbors {
            dist[nx][ny] = dist[x][y] + 1;
            queue.push_back((nx, ny));
        }
    }

    dist
}

fn reachable_cells(grid: &Grid, start_dist: &[Vec<usize>]) -> Vec<(usize, usize)> {
    iproduct!(0..grid.len(), 0..grid[0].len())
        .filter(|&(x, y)| start_dist[x][y] != usize::MAX && is_track(grid, x, y))
        .collect()
}

fn cheats(
    grid: &Grid,
    start_dist: &[Vec<usize>],
    end_dist: &[Vec<usize>],
    normal_dist: usize,
    steps: Option<usize>,
) -> HashSet<(usize, usize, usize, usize)> {
    let mut cheats = HashSet::new();
    let cells = reachable_cells(grid, start_dist);

    for &(sx, sy) in &cells {
        let base_dist = start_dist[sx][sy];
        let dist_no_walls = match steps {
            Some(max_steps) => bfs(grid, (sx, sy), Some(max_steps), false),
            None => {
                let mut dist = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
                for (dx1, dy1) in DIRECTIONS {
                    let nx = (sx as i32 + dx1) as usize;
                    let ny = (sy as i32 + dy1) as usize;
                    if nx < grid.len() && ny < grid[0].len() {
                        dist[nx][ny] = 1;

                        for (dx2, dy2) in DIRECTIONS {
                            let fx = (nx as i32 + dx2) as usize;
                            let fy = (ny as i32 + dy2) as usize;
                            if fx < grid.len() && fy < grid[0].len() && is_track(grid, fx, fy) {
                                dist[fx][fy] = 2;
                            }
                        }
                    }
                }
                dist
            }
        };

        iproduct!(0..grid.len(), 0..grid[0].len())
            .filter(|&(fx, fy)| {
                let d = dist_no_walls[fx][fy];
                d != usize::MAX
                    && match steps {
                        Some(max_steps) => (1..=max_steps).contains(&d),
                        None => d <= 2,
                    }
                    && is_track(grid, fx, fy)
                    && end_dist[fx][fy] != usize::MAX
            })
            .for_each(|(fx, fy)| {
                let route_with_cheat = base_dist + dist_no_walls[fx][fy] + end_dist[fx][fy];
                let saving = normal_dist.saturating_sub(route_with_cheat);
                if saving >= 100 {
                    cheats.insert((sx, sy, fx, fy));
                }
            });
    }

    cheats
}

fn solve(input: &Maze, steps: Option<usize>) -> String {
    let start_dist = bfs(&input.grid, input.start, None, true);
    let end_dist = bfs(&input.grid, input.end, None, true);

    let normal_dist = start_dist[input.end.0][input.end.1];
    if normal_dist == usize::MAX {
        return "0".to_string();
    }

    let cheats = cheats(&input.grid, &start_dist, &end_dist, normal_dist, steps);
    cheats.len().to_string()
}

pub fn part1(input: &Maze) -> String {
    solve(input, None)
}

pub fn part2(input: &Maze) -> String {
    solve(input, Some(20))
}

#[test]
fn test_day20() {
    let input = parse(
        "###############
         #...#...#.....#
         #.#.#.#.#.###.#
         #S#...#.#.#...#
         #######.#.#.###
         #######.#.#...#
         #######.#.###.#
         ###..E#...#...#
         ###.#######.###
         #...###...#...#
         #.#####.#.###.#
         #.#...#.#.#...#
         #.#.#.#.#.#.###
         #...#...#...###
         ###############",
    );
    assert_eq!(part1(&input), "0");
    assert_eq!(part2(&input), "0");
}
