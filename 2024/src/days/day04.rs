use itertools::iproduct;

type Grid = Vec<Vec<char>>;
type Point = (usize, usize);
type Direction = (isize, isize);

pub fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn check_pattern(grid: &Grid, start: Point, (di, dj): Direction, pattern: &str) -> bool {
    let (rows, cols) = (grid.len(), grid[0].len());

    pattern.chars().enumerate().all(|(k, target)| {
        let row = start.0.checked_add_signed(di * k as isize);
        let col = start.1.checked_add_signed(dj * k as isize);

        matches!((row, col), (Some(r), Some(c)) if r < rows && c < cols && grid[r][c] == target)
    })
}

fn count_xmas(grid: &Grid) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let directions: [Direction; 8] = [
        (0, 1),
        (1, 0),
        (1, 1),
        (-1, 1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (1, -1),
    ];

    iproduct!(0..rows, 0..cols, &directions)
        .filter(|&(i, j, &dir)| check_pattern(grid, (i, j), dir, "XMAS"))
        .count()
}

fn count_x_mas(grid: &Grid) -> usize {
    let patterns = [
        ("MAS", "MAS"),
        ("MAS", "SAM"),
        ("SAM", "MAS"),
        ("SAM", "SAM"),
    ];

    iproduct!(1..grid.len() - 1, 1..grid[0].len() - 1)
        .filter(|&(i, j)| grid[i][j] == 'A')
        .flat_map(|(i, j)| {
            patterns.iter().filter(move |&&(p1, p2)| {
                check_pattern(grid, (i - 1, j - 1), (1, 1), p1)
                    && check_pattern(grid, (i - 1, j + 1), (1, -1), p2)
            })
        })
        .count()
}

pub fn part1(input: &Grid) -> String {
    count_xmas(input).to_string()
}

pub fn part2(input: &Grid) -> String {
    count_x_mas(input).to_string()
}

#[test]
fn test_day04() {
    let input = parse(
        "MMMSXXMASM
                MSAMXMSMSA
                AMXSXMAAMM
                MSAMASMSMX
                XMASAMXAMM
                XXAMMXXAMA
                SMSMSASXSS
                SAXAMASAAA
                MAMMMXMMMM
                MXMXAXMASX",
    );
    assert_eq!(part1(&input), "18");
    assert_eq!(part2(&input), "9");
}
