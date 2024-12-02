use super::Day;

pub struct Day02;

impl Day02 {
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
        Self::is_safe(nums)
            || nums.iter().enumerate().any(|(i, _)| {
                Self::is_safe(
                    &nums[..i]
                        .iter()
                        .chain(nums[i + 1..].iter())
                        .copied()
                        .collect::<Vec<_>>(),
                )
            })
    }
}

impl Day<Vec<Vec<u32>>> for Day02 {
    fn parse(&self, input: &str) -> Vec<Vec<u32>> {
        input
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|part| part.parse().unwrap())
                    .collect()
            })
            .collect()
    }

    fn part1(&self, input: &Vec<Vec<u32>>) -> String {
        input
            .iter()
            .filter(|nums| Self::is_safe(nums))
            .count()
            .to_string()
    }

    fn part2(&self, input: &Vec<Vec<u32>>) -> String {
        input
            .iter()
            .filter(|nums| Self::dampener_safe(nums))
            .count()
            .to_string()
    }
}
