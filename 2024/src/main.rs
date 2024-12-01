use days::Day;

mod days;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day = &args[1];

    let input = std::fs::read_to_string(format!("input/day{day}.txt").as_str()).unwrap();

    let module = match day.as_str() {
        "1" => days::day01::Day01,
        _ => panic!("Day not implemented"),
    };

    let parsed = module.parse(&input);

    println!("Day {day}");
    println!("Part 1: {}", module.part1(&parsed));
    println!("Part 2: {}", module.part2(&parsed));
}
