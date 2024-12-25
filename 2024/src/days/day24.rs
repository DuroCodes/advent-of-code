use itertools::Itertools;
use std::collections::HashMap;

enum Gate {
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

pub struct Circuit {
    wires: HashMap<String, bool>,
    gates: HashMap<String, Gate>,
}

pub fn parse(input: &str) -> Circuit {
    let (initial, gates) = input.split("\n\n").collect_tuple().unwrap();

    let wires = initial
        .lines()
        .map(str::trim)
        .filter_map(|line| line.split(": ").collect_tuple())
        .map(|(key, val)| (key.to_string(), val == "1"))
        .collect();

    let gates = gates
        .lines()
        .map(str::trim)
        .filter_map(|line| {
            let (expr, out) = line.split(" -> ").collect_tuple()?;
            let (l, op, r) = expr.split(' ').collect_tuple()?;
            let gate = match op {
                "AND" => Gate::And(l.to_string(), r.to_string()),
                "OR" => Gate::Or(l.to_string(), r.to_string()),
                "XOR" => Gate::Xor(l.to_string(), r.to_string()),
                _ => return None,
            };
            Some((out.to_string(), gate))
        })
        .collect();

    Circuit { wires, gates }
}

fn wire_type(wire: &str) -> Option<(char, usize)> {
    Some((wire.chars().next()?, wire.get(1..)?.parse::<usize>().ok()?))
}

pub fn part1(input: &Circuit) -> String {
    let mut wires = input.wires.clone();

    while let Some((output, value)) = input
        .gates
        .iter()
        .filter(|(out, _)| !wires.contains_key(*out))
        .find_map(|(out, gate)| {
            let (l, r) = match gate {
                Gate::And(l, r) | Gate::Or(l, r) | Gate::Xor(l, r) => (l, r),
            };
            wires.get(l).zip(wires.get(r)).map(|(&l_val, &r_val)| {
                let val = match gate {
                    Gate::And(_, _) => l_val & r_val,
                    Gate::Or(_, _) => l_val | r_val,
                    Gate::Xor(_, _) => l_val ^ r_val,
                };
                (out.clone(), val)
            })
        })
    {
        wires.insert(output, value);
    }

    (0..)
        .take_while(|i| wires.contains_key(&format!("z{:02}", i)))
        .map(|i| (wires[&format!("z{:02}", i)] as u64) << i)
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: &Circuit) -> String {
    let (min_bit, max_bit) = input
        .wires
        .keys()
        .filter_map(|w| w[1..].parse::<usize>().ok())
        .fold((usize::MAX, 0), |(min, max), n| {
            (min.min(n), max.max(n + 1))
        });

    let xy_wire = |w: &str| wire_type(w).map_or(false, |(t, p)| "xy".contains(t) && p > min_bit);

    let used_in_gates = |output: &str, gate_type: fn(&Gate) -> bool| {
        input.gates.values().any(|g| match g {
            g if gate_type(g) => match g {
                Gate::And(a, b) | Gate::Or(a, b) | Gate::Xor(a, b) => a == output || b == output,
            },
            _ => false,
        })
    };

    input
        .gates
        .iter()
        .filter(|(out, gate)| {
            let (l, r) = match gate {
                Gate::And(l, r) | Gate::Or(l, r) | Gate::Xor(l, r) => (l, r),
            };

            match wire_type(out) {
                Some(('z', pos)) if pos < max_bit => !matches!(gate, Gate::Xor(_, _)),
                _ => {
                    let both_xy = xy_wire(l) && xy_wire(r);
                    match gate {
                        Gate::Xor(_, _) if !both_xy => true,
                        Gate::Xor(_, _) if both_xy => {
                            !used_in_gates(out, |g| matches!(g, Gate::Xor(_, _)))
                        }
                        Gate::And(_, _) if both_xy => {
                            !used_in_gates(out, |g| matches!(g, Gate::Or(_, _)))
                        }
                        _ => false,
                    }
                }
            }
        })
        .map(|(out, _)| out)
        .sorted()
        .join(",")
}

#[test]
fn test_day24() {
    let input = parse(
        "x00: 1
         x01: 1
         x02: 1
         y00: 0
         y01: 1
         y02: 0

         x00 AND y00 -> z00
         x01 XOR y01 -> z01
         x02 OR y02 -> z02",
    );
    assert_eq!(part1(&input), "4");
    assert_eq!(part2(&input), "z00,z02");
}
