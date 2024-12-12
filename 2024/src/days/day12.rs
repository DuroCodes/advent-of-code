use itertools::Itertools;
use std::collections::{BTreeSet, HashSet};

type Grid = Vec<Vec<char>>;
type Point = (i32, i32);
type Item = (Point, usize);

const DX: [i32; 4] = [-1, 0, 1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

pub fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn find_region(grid: &Grid, start: Point, visited: &mut HashSet<Point>) -> (usize, usize) {
    let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
    let target = grid[start.0 as usize][start.1 as usize];
    let mut stack = vec![start];
    let mut area = 1;
    let mut perimeter = 0;
    visited.insert(start);

    while let Some((r, c)) = stack.pop() {
        let neighbors: Vec<_> = DX
            .iter()
            .zip(DY.iter())
            .map(|(&dx, &dy)| (r + dx, c + dy))
            .collect();

        perimeter += neighbors
            .iter()
            .filter(|&&(nr, nc)| {
                !(0..rows).contains(&nr)
                    || !(0..cols).contains(&nc)
                    || grid[nr as usize][nc as usize] != target
            })
            .count();

        let valid_neighbors: Vec<_> = neighbors
            .into_iter()
            .filter(|&(nr, nc)| {
                (0..rows).contains(&nr)
                    && (0..cols).contains(&nc)
                    && grid[nr as usize][nc as usize] == target
                    && !visited.contains(&(nr, nc))
            })
            .collect();

        for pos in valid_neighbors {
            visited.insert(pos);
            stack.push(pos);
            area += 1;
        }
    }

    (area, perimeter)
}

fn connected_segments(mut boundary: BTreeSet<Item>) -> usize {
    let mut count = 0;
    while !boundary.is_empty() {
        count += 1;
        let mut current = *boundary.iter().next().unwrap();
        boundary.remove(&current);

        while let Some(&next) = boundary.iter().find(|&&item| {
            (0..4).any(|d| {
                item.0 .0 == current.0 .0 + DX[d]
                    && item.0 .1 == current.0 .1 + DY[d]
                    && item.1 == current.1
            })
        }) {
            boundary.remove(&next);
            current = next;
        }
    }

    count
}

fn component(input: &Grid, visited: &mut Vec<Vec<bool>>, start: (usize, usize)) -> Vec<Point> {
    let (n, m) = (input.len() as i32, input[0].len() as i32);
    let target = input[start.0][start.1];
    let mut component = Vec::new();
    let mut stack = vec![(start.0 as i32, start.1 as i32)];
    visited[start.0][start.1] = true;

    while let Some(pos) = stack.pop() {
        component.push(pos);
        for d in 0..4 {
            let (nx, ny) = (pos.0 + DX[d], pos.1 + DY[d]);
            if (0..n).contains(&nx)
                && (0..m).contains(&ny)
                && !visited[nx as usize][ny as usize]
                && input[nx as usize][ny as usize] == target
            {
                visited[nx as usize][ny as usize] = true;
                stack.push((nx, ny));
            }
        }
    }
    component
}

fn boundary(input: &Grid, component: &[Point], target: char) -> BTreeSet<Item> {
    let (n, m) = (input.len() as i32, input[0].len() as i32);
    component
        .iter()
        .flat_map(|&(i, j)| (0..4).map(move |d| ((i + DX[d], j + DY[d]), d)))
        .filter(|&(pos, _)| {
            !(0..n).contains(&pos.0)
                || !(0..m).contains(&pos.1)
                || input[pos.0 as usize][pos.1 as usize] != target
        })
        .collect()
}

fn component_value(input: &Grid, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> usize {
    if visited[i][j] {
        return 0;
    }

    let component = component(input, visited, (i, j));
    let boundary = boundary(input, &component, input[i][j]);

    component.len() * connected_segments(boundary)
}

pub fn part1(input: &Grid) -> String {
    let mut visited = HashSet::new();

    (0..input.len())
        .cartesian_product(0..input[0].len())
        .map(|(r, c)| match visited.contains(&(r as i32, c as i32)) {
            true => 0,
            false => {
                let (area, perimeter) = find_region(input, (r as i32, c as i32), &mut visited);
                area * perimeter
            }
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &Grid) -> String {
    let (n, m) = (input.len(), input[0].len());
    let mut visited = vec![vec![false; m]; n];

    (0..n)
        .cartesian_product(0..m)
        .map(|(i, j)| component_value(input, &mut visited, i, j))
        .sum::<usize>()
        .to_string()
}

#[test]
fn test_day12() {
    let input = parse(
        "AAAA
         BBCD
         BBCC
         EEEC",
    );
    assert_eq!(part1(&input), "140");
    assert_eq!(part2(&input), "80");
}
