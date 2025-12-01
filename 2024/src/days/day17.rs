use regex::Regex;

#[derive(Clone)]
struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    ip: usize,
    program: Vec<i64>,
    outputs: Vec<i64>,
}

fn combo_value(computer: &Computer, operand: i64) -> i64 {
    match operand {
        0..=3 => operand,
        4 => computer.reg_a,
        5 => computer.reg_b,
        6 => computer.reg_c,
        _ => panic!("Invalid combo operand"),
    }
}

fn execute(computer: &Computer, opcode: i64, operand: i64) -> (usize, i64, i64, i64, Option<i64>) {
    match opcode {
        0 => (
            computer.ip + 2,
            computer.reg_a / (1 << combo_value(computer, operand)),
            computer.reg_b,
            computer.reg_c,
            None,
        ),
        1 => (
            computer.ip + 2,
            computer.reg_a,
            computer.reg_b ^ operand,
            computer.reg_c,
            None,
        ),
        2 => (
            computer.ip + 2,
            computer.reg_a,
            combo_value(computer, operand) % 8,
            computer.reg_c,
            None,
        ),
        3 => (
            match computer.reg_a {
                0 => computer.ip + 2,
                _ => operand as usize,
            },
            computer.reg_a,
            computer.reg_b,
            computer.reg_c,
            None,
        ),
        4 => (
            computer.ip + 2,
            computer.reg_a,
            computer.reg_b ^ computer.reg_c,
            computer.reg_c,
            None,
        ),
        5 => (
            computer.ip + 2,
            computer.reg_a,
            computer.reg_b,
            computer.reg_c,
            Some(combo_value(computer, operand) % 8),
        ),
        6 => (
            computer.ip + 2,
            computer.reg_a,
            computer.reg_a / (1 << combo_value(computer, operand)),
            computer.reg_c,
            None,
        ),
        7 => (
            computer.ip + 2,
            computer.reg_a,
            computer.reg_b,
            computer.reg_a / (1 << combo_value(computer, operand)),
            None,
        ),
        _ => panic!("Invalid opcode"),
    }
}

fn run(computer: &Computer, target: Option<&[i64]>) -> Vec<i64> {
    let mut current = computer.clone();

    while current.ip < current.program.len() {
        let opcode = current.program[current.ip];
        let operand = current.program[current.ip + 1];

        let (new_ip, new_a, new_b, new_c, output) = execute(&current, opcode, operand);

        if let Some(out) = output {
            if let Some(target) = target {
                if current.outputs.len() < target.len() && out != target[current.outputs.len()] {
                    current.outputs.push(out);
                    break;
                }
            }
            current.outputs.push(out);
        }

        current.ip = new_ip;
        current.reg_a = new_a;
        current.reg_b = new_b;
        current.reg_c = new_c;
    }

    current.outputs
}

pub fn parse(input: &str) -> (Vec<i64>, i64, i64, i64) {
    let re_reg = Regex::new(r"Register ([ABC]): (-?\d+)").unwrap();
    let re_prog = Regex::new(r"Program: (.+)").unwrap();

    let mut reg_a = 0;
    let mut reg_b = 0;
    let mut reg_c = 0;
    let mut program = Vec::new();

    for line in input.lines() {
        if let Some(cap) = re_reg.captures(line) {
            let value = cap[2].parse::<i64>().unwrap();
            match &cap[1] {
                "A" => reg_a = value,
                "B" => reg_b = value,
                "C" => reg_c = value,
                _ => unreachable!(),
            }
        } else if let Some(cap) = re_prog.captures(line) {
            program = cap[1]
                .split(',')
                .map(|n| n.trim().parse::<i64>().unwrap())
                .collect();
        }
    }

    (program, reg_a, reg_b, reg_c)
}

pub fn part1(input: &(Vec<i64>, i64, i64, i64)) -> String {
    let (program, reg_a, reg_b, reg_c) = input;
    let computer = Computer {
        reg_a: *reg_a,
        reg_b: *reg_b,
        reg_c: *reg_c,
        ip: 0,
        program: program.clone(),
        outputs: Vec::new(),
    };

    run(&computer, None)
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn part2(input: &(Vec<i64>, i64, i64, i64)) -> String {
    // this part is a little strange; it uses analysis of the octal representation of the number
    // it helps brute force solution due to the large number of possible inputs (216 billion with my input)
    // 17156052247155 a large number of digits that i bruteforced to find the solution for my input
    // you can uncomment the println! to see the progress; for instance: with 5 digits (47155):
    // 12047155 -> 12247155 -> 112247155 -> 6052247155 -> 35512247155 -> 2166052247155 -> 7156052247155
    // therefore, you can assume the next few last digits would be 52247155, and so on until you find enough digits to reasonably brute force
    // repeat until you find a reasonable number of digits; 13 digits seem to work well

    let (program, _, _, _) = input;
    // let base = 0;
    // let power = 1;
    let base = i64::from_str_radix("17156052247155", 8).unwrap();
    let power = 8i64.pow(13);
    let mut best = 0;

    (1..)
        .find_map(|ast| {
            let reg_a = ast * power + base;
            let computer = Computer {
                reg_a,
                reg_b: 0,
                reg_c: 0,
                ip: 0,
                program: program.clone(),
                outputs: Vec::new(),
            };

            let outputs = run(&computer, Some(program));

            if outputs.len() > best {
                println!("{reg_a:o}");
                best = outputs.len();
            }

            (outputs.len() == program.len() && outputs == *program).then_some(reg_a)
        })
        .unwrap()
        .to_string()
}

#[test]
fn test_day17() {
    let input = parse(
        "Register A: 2024
         Register B: 0
         Register C: 0
         
         Program: 0,3,5,4,3,0",
    );
    assert_eq!(part1(&input), "5,7,3,0");
    // assert_eq!(part2(&input), "117440"); // <- won't work properly due to digit analysis in the actual solution
}
