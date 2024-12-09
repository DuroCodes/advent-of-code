pub fn parse(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, len)| {
            acc.extend(
                std::iter::repeat(if i % 2 == 0 { i as i32 / 2 } else { -1 }).take(len as usize),
            );
            acc
        })
}

fn compact(blocks: &[i32]) -> Vec<i32> {
    let mut result = blocks.to_vec();

    result
        .iter()
        .enumerate()
        .filter(|(_, &x)| x == -1)
        .map(|(i, _)| i)
        .collect::<Vec<_>>()
        .iter()
        .for_each(|&gap_pos| {
            if let Some(file_pos) = result[gap_pos..]
                .iter()
                .enumerate()
                .filter(|(_, &id)| id != -1)
                .map(|(i, _)| i + gap_pos)
                .last()
            {
                result.swap(gap_pos, file_pos);
            }
        });

    result
}

fn compact_whole(blocks: &[i32]) -> Vec<i32> {
    let max_file_id = blocks
        .iter()
        .filter(|&&x| x >= 0)
        .max()
        .copied()
        .unwrap_or(0);

    (0..=max_file_id)
        .rev()
        .fold(blocks.to_vec(), |acc, file_id| {
            let file_positions: Vec<usize> = acc
                .iter()
                .enumerate()
                .filter(|(_, &id)| id == file_id)
                .map(|(i, _)| i)
                .collect();

            if file_positions.is_empty() {
                return acc;
            }

            let file_size = file_positions.len();
            let mut result = acc;

            let gaps = result
                .iter()
                .enumerate()
                .scan((None, 0), |(start, size), (i, &block)| {
                    match block {
                        -1 => {
                            if start.is_none() {
                                *start = Some(i);
                            }
                            *size += 1;
                        }
                        _ => {
                            *start = None;
                            *size = 0;
                        }
                    }
                    Some((*start, *size))
                })
                .filter(|&(start, size)| size >= file_size && start.unwrap() < file_positions[0])
                .next();

            if let Some((Some(gap_start), _)) = gaps {
                file_positions
                    .iter()
                    .enumerate()
                    .for_each(|(offset, &src_pos)| {
                        result[gap_start + offset] = file_id;
                        result[src_pos] = -1;
                    });
            }

            result
        })
}

fn checksum(blocks: &[i32]) -> i64 {
    blocks
        .iter()
        .enumerate()
        .filter(|(_, &id)| id != -1)
        .map(|(pos, &id)| pos as i64 * id as i64)
        .sum()
}

pub fn part1(blocks: &[i32]) -> String {
    checksum(&compact(blocks)).to_string()
}

pub fn part2(blocks: &[i32]) -> String {
    checksum(&compact_whole(blocks)).to_string()
}

#[test]
fn test_day09() {
    let input = parse("2333133121414131402");
    assert_eq!(part1(&input), "1928");
    assert_eq!(part2(&input), "2858");
}
