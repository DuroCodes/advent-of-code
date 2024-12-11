use std::collections::HashSet;

type Grid = Vec<Vec<u8>>;
type Pos = (usize, usize);

pub fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn trailheads(grid: &Grid) -> Vec<Pos> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &height)| height == 0)
                .map(move |(x, _)| (x, y))
        })
        .collect()
}

fn neighbors(pos: Pos, grid: &Grid) -> Vec<Pos> {
    let (x, y) = pos;
    let height = grid[y][x];

    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .into_iter()
        .filter_map(|(dx, dy)| {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            (nx >= 0 && ny >= 0 && ny < grid.len() as i32 && nx < grid[0].len() as i32)
                .then(|| (nx as usize, ny as usize))
                .filter(|&(nx, ny)| grid[ny][nx] == height + 1)
        })
        .collect()
}

fn traverse(pos: Pos, grid: &Grid, visited: &mut HashSet<Pos>) {
    if !visited.insert(pos) {
        return;
    }

    neighbors(pos, grid)
        .into_iter()
        .for_each(|next| traverse(next, grid, visited));
}

fn reachable_positions(start: Pos, grid: &Grid) -> HashSet<Pos> {
    let mut visited = HashSet::new();
    traverse(start, grid, &mut visited);
    visited
}

fn reachable_nines(start: Pos, grid: &Grid) -> usize {
    reachable_positions(start, grid)
        .into_iter()
        .filter(|&(x, y)| grid[y][x] == 9)
        .count()
}

fn count_paths(pos: Pos, grid: &Grid, visited: &mut HashSet<Pos>) -> usize {
    if grid[pos.1][pos.0] == 9 {
        return 1;
    }

    visited.insert(pos);

    let valid_neighbors: Vec<_> = neighbors(pos, grid)
        .into_iter()
        .filter(|pos| !visited.contains(pos))
        .collect();

    let paths = valid_neighbors
        .into_iter()
        .map(|next| count_paths(next, grid, visited))
        .sum();

    visited.remove(&pos);
    paths
}

pub fn part1(input: &Grid) -> String {
    trailheads(input)
        .into_iter()
        .map(|pos| reachable_nines(pos, input))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &Grid) -> String {
    trailheads(input)
        .into_iter()
        .map(|pos| count_paths(pos, input, &mut HashSet::new()))
        .sum::<usize>()
        .to_string()
}

#[test]
fn test_day10() {
    let input = parse(
        "89010123
         78121874
         87430965
         96549874
         45678903
         32019012
         01329801
         10456732",
    );
    assert_eq!(part1(&input), "36");
    assert_eq!(part2(&input), "81");
}
