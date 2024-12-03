use std::env;
use std::fs;

mod days;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        std::process::exit(1);
    }

    let day: usize = args[1].parse().expect("Day must be a number");
    if day == 0 || day > days::SOLUTIONS.len() {
        panic!("Day {day} not implemented");
    }

    let input = match args.len() {
        3 => fs::read_to_string(&args[2]).expect("Failed to read input"),
        _ => fs::read_to_string(format!("input/day{day}.txt")).expect("Failed to read input"),
    };

    let (part1, part2) = days::SOLUTIONS[day - 1];

    println!("Day {day}");

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&input));
    println!("Part 1 took: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(&input));
    println!("Part 2 took: {:?}", start.elapsed());
}
