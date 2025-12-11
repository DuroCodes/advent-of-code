use std::collections::HashMap;

pub type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

pub fn parse(input: &str) -> Graph<'_> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| line.split_once(':'))
        .map(|(device, outputs)| (device.trim(), outputs.split_whitespace().collect()))
        .collect()
}

fn count_paths<'a>(
    graph: &Graph<'a>,
    start: &'a str,
    end: &str,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    (start == end)
        .then_some(1)
        .or_else(|| memo.get(start).copied())
        .unwrap_or_else(|| {
            let count = graph
                .get(start)
                .map(|neighbors| {
                    neighbors
                        .iter()
                        .map(|&n| count_paths(graph, n, end, memo))
                        .sum()
                })
                .unwrap_or(0);
            memo.insert(start, count);
            count
        })
}

fn count_paths_through<'a>(
    graph: &Graph<'a>,
    node: &'a str,
    end: &str,
    state: (bool, bool),
    required: (&str, &str),
    memo: &mut HashMap<(&'a str, bool, bool), usize>,
) -> usize {
    let state = (state.0 || node == required.0, state.1 || node == required.1);

    (node == end)
        .then(|| usize::from(state.0 && state.1))
        .or_else(|| memo.get(&(node, state.0, state.1)).copied())
        .unwrap_or_else(|| {
            let count = graph
                .get(node)
                .map(|neighbors| {
                    neighbors
                        .iter()
                        .map(|&n| count_paths_through(graph, n, end, state, required, memo))
                        .sum()
                })
                .unwrap_or(0);
            memo.insert((node, state.0, state.1), count);
            count
        })
}

pub fn part1(graph: &Graph) -> String {
    count_paths(graph, "you", "out", &mut HashMap::new()).to_string()
}

pub fn part2(graph: &Graph) -> String {
    count_paths_through(
        graph,
        "svr",
        "out",
        (false, false),
        ("dac", "fft"),
        &mut HashMap::new(),
    )
    .to_string()
}

#[test]
fn test_day11() {
    let input = parse("aaa: you hhh\nyou: bbb ccc\nbbb: ddd eee\nccc: ddd eee fff\nddd: ggg\neee: out\nfff: out\nggg: out\nhhh: ccc fff iii\niii: out");
    let input2 = parse("svr: aaa bbb\naaa: fft\nfft: ccc\nbbb: tty\ntty: ccc\nccc: ddd eee\nddd: hub\nhub: fff\neee: dac\ndac: fff\nfff: ggg hhh\nggg: out\nhhh: out");

    assert_eq!(part1(&input), "5");
    assert_eq!(part2(&input2), "2");
}
