fn is_safe(nums: &[u32]) -> bool {
    nums.windows(2).next().map_or(true, |w| {
        let increasing = w[0] < w[1];
        nums.windows(2).all(|w| {
            let diff = w[0].abs_diff(w[1]);
            diff >= 1 && diff <= 3 && (w[0] < w[1]) == increasing
        })
    })
}

fn dampener_safe(nums: &[u32]) -> bool {
    is_safe(nums)
        || nums.iter().enumerate().any(|(i, _)| {
            is_safe(
                &nums[..i]
                    .iter()
                    .chain(nums[i + 1..].iter())
                    .copied()
                    .collect::<Vec<_>>(),
            )
        })
}

pub fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|part| part.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &[Vec<u32>]) -> String {
    input
        .iter()
        .filter(|nums| is_safe(nums))
        .count()
        .to_string()
}

pub fn part2(input: &[Vec<u32>]) -> String {
    input
        .iter()
        .filter(|nums| dampener_safe(nums))
        .count()
        .to_string()
}
