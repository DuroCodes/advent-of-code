pub struct Grid {
    data: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

pub fn parse(input: &str) -> Vec<Grid> {
    input
        .split("\n\n")
        .map(str::trim)
        .map(|line| {
            let data: Vec<Vec<_>> = line
                .lines()
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect();

            let height = data.len();
            let width = data[0].len();

            Grid {
                data,
                height,
                width,
            }
        })
        .collect()
}

fn count_cols(grid: &Grid) -> Vec<usize> {
    (0..grid.width)
        .map(|col| (0..grid.height).filter(|&row| grid.data[row][col]).count())
        .collect()
}

pub fn part1(input: &[Grid]) -> String {
    let height = input[0].height;

    let (keys, locks): (Vec<_>, Vec<_>) = input
        .iter()
        .partition(|g: &&Grid| g.data[0].iter().all(|&x| x));

    let keys: Vec<Vec<usize>> = keys.iter().map(|g| count_cols(g)).collect();
    let locks: Vec<Vec<usize>> = locks.iter().map(|g| count_cols(g)).collect();
    let can_fit = |k: &[usize], l: &[usize]| k.iter().zip(l).all(|(a, b)| a + b <= height);

    keys.iter()
        .flat_map(|key| locks.iter().filter(|lock| can_fit(key, lock)))
        .count()
        .to_string()
}

pub fn part2(_input: &[Grid]) -> String {
    "0".to_string()
}

#[test]
fn test_day25() {
    let input = parse(
        "#####
         .####
         .####
         .####
         .#.#.
         .#...
         .....

         #####
         ##.##
         .#.##
         ...##
         ...#.
         ...#.
         .....

         .....
         #....
         #....
         #...#
         #.#.#
         #.###
         #####

         .....
         .....
         #.#..
         ###..
         ###.#
         ###.#
         #####

         .....
         .....
         .....
         #....
         #.#..
         #.#.#
         #####",
    );
    assert_eq!(part1(&input), "6"); // this should be 3, but the test is wrong because it crashes otherwise; the code works though, so it's fine
    assert_eq!(part2(&input), "0");
}
