use days::Solution;

mod days;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day = &args[1];

    let input = std::fs::read_to_string(format!("input/day{day}.txt").as_str()).unwrap();

    let solution = match day.as_str() {
        "1" => Solution::Day01(days::day01::Day01),
        "2" => Solution::Day02(days::day02::Day02),
        _ => panic!("Day not implemented"),
    };

    let (part1, part2) = solution.run(&input);
    println!("Day {day}");
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
