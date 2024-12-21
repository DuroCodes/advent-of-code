use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: i64,
    pos: (i32, i32),
    prev: char,
    out: String,
}

impl State {
    fn new(cost: i64, pos: (i32, i32), prev: char, out: String) -> Self {
        Self {
            cost,
            pos,
            prev,
            out,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, PartialEq)]
enum PadType {
    Numeric,
    Directional,
}

fn pad(pos: (i32, i32), pad_type: PadType) -> Option<char> {
    let keypad = match pad_type {
        PadType::Numeric => "789 456 123  0A",
        PadType::Directional => " ^A <v>",
    };

    keypad
        .chars()
        .nth((4 * pos.0 + pos.1) as usize)
        .filter(|&c| c != ' ')
}

fn apply((r, c): (i32, i32), mv: char, pad_type: PadType) -> ((i32, i32), Option<char>) {
    match mv {
        'A' => ((r, c), pad((r, c), pad_type)),
        '<' => ((r, c - 1), None),
        '^' => ((r - 1, c), None),
        '>' => ((r, c + 1), None),
        'v' => ((r + 1, c), None),
        _ => unreachable!(),
    }
}

fn extend(
    queue: &mut BinaryHeap<State>,
    state: &State,
    pad_type: PadType,
    pads: i32,
    dp: &mut HashMap<(char, char, i32), i64>,
) {
    queue.extend(['A', '<', '>', 'v', '^'].iter().map(|&mv| {
        let (pos, output) = apply(state.pos, mv, pad_type);
        let cost = match pad_type {
            PadType::Directional => cost(mv, state.prev, pads - 1, dp),
            PadType::Numeric => cost(mv, state.prev, pads, dp),
        };
        let out = match output {
            Some(out) => format!("{}{}", state.out, out),
            None => state.out.clone(),
        };
        State::new(state.cost + cost, pos, mv, out)
    }));
}

fn cost(ch: char, prev: char, pads: i32, dp: &mut HashMap<(char, char, i32), i64>) -> i64 {
    match (dp.get(&(ch, prev, pads)), pads) {
        (Some(&cost), _) => return cost,
        (_, 0) => return 1,
        _ => {}
    }

    let start_pos = match prev {
        '^' => (0, 1),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        'A' => (0, 2),
        _ => unreachable!(),
    };

    let mut queue = BinaryHeap::new();
    let mut seen = HashMap::new();
    queue.push(State::new(0, start_pos, 'A', String::new()));

    while let Some(state) = queue.pop() {
        if pad(state.pos, PadType::Directional).is_none() {
            continue;
        }

        if state.out == ch.to_string() {
            let cost = state.cost;
            dp.insert((ch, prev, pads), cost);
            return cost;
        }

        if !state.out.is_empty() {
            continue;
        }

        let key = (state.pos, state.prev);
        if seen.get(&key).map_or(false, |&c| state.cost >= c) {
            continue;
        }

        seen.insert(key, state.cost);
        extend(&mut queue, &state, PadType::Directional, pads, dp);
    }
    unreachable!()
}

fn solve(code: &str, pads: i32) -> i64 {
    let mut queue = BinaryHeap::new();
    let mut seen = HashMap::new();
    let mut dp = HashMap::new();

    queue.push(State::new(0, (3, 2), 'A', String::new()));

    while let Some(state) = queue.pop() {
        if state.out == code {
            return state.cost;
        }

        if !code.starts_with(&state.out) || pad(state.pos, PadType::Numeric).is_none() {
            continue;
        }

        let key = (state.pos, state.prev, state.out.clone());
        if seen.contains_key(&key) {
            continue;
        }

        seen.insert(key, state.cost);
        extend(&mut queue, &state, PadType::Numeric, pads, &mut dp);
    }
    unreachable!()
}

pub fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.trim().to_string()).collect()
}

pub fn solve_part(input: &[String], multiplier: i32) -> String {
    input
        .iter()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<i64>()
                .unwrap()
                * solve(line, multiplier)
        })
        .sum::<i64>()
        .to_string()
}

pub fn part1(input: &[String]) -> String {
    solve_part(input, 2)
}

pub fn part2(input: &[String]) -> String {
    solve_part(input, 25)
}

#[test]
fn test_day21() {
    let input = parse("029A\n980A\n179A\n456A\n379A");
    assert_eq!(part1(&input), "126384");
    assert_eq!(part2(&input), "154115708116294");
}
