use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Graph = HashMap<String, HashSet<String>>;

pub fn parse(input: &str) -> Graph {
    input.lines().filter_map(|line| line.split_once('-')).fold(
        HashMap::new(),
        |mut graph, (a, b)| {
            [(a, b), (b, a)].into_iter().for_each(|(from, to)| {
                graph
                    .entry(from.to_string())
                    .or_default()
                    .insert(to.to_string());
            });
            graph
        },
    )
}

fn triplets(graph: &Graph) -> Vec<HashSet<String>> {
    graph
        .keys()
        .tuple_combinations()
        .filter(|(i, j, k)| {
            graph[*i].contains(*j) && graph[*i].contains(*k) && graph[*j].contains(*k)
        })
        .map(|(i, j, k)| {
            [i.clone(), j.clone(), k.clone()]
                .into_iter()
                .collect::<HashSet<_>>()
        })
        .collect()
}

// bron-kerbosch
fn max_clique(
    graph: &Graph,
    clique: HashSet<String>,
    candidates: HashSet<String>,
    excluded: HashSet<String>,
) -> HashSet<String> {
    if candidates.is_empty() && excluded.is_empty() {
        return clique;
    }

    if candidates.is_empty() {
        return HashSet::new();
    }

    let v = candidates.iter().next().unwrap();
    let neighbors: HashSet<_> = graph[v].iter().cloned().collect();

    let neighbor_intersect = |set: &HashSet<String>| {
        set.iter()
            .filter(|u| neighbors.contains(*u))
            .cloned()
            .collect::<HashSet<_>>()
    };

    let with_v = max_clique(
        graph,
        clique.iter().chain(std::iter::once(v)).cloned().collect(),
        neighbor_intersect(&candidates),
        neighbor_intersect(&excluded),
    );

    let without_v = max_clique(
        graph,
        clique,
        candidates.iter().filter(|&u| u != v).cloned().collect(),
        excluded.iter().chain(std::iter::once(v)).cloned().collect(),
    );

    match with_v.len() > without_v.len() {
        true => with_v,
        false => without_v,
    }
}

pub fn part1(input: &Graph) -> String {
    triplets(input)
        .iter()
        .filter(|triplet| triplet.iter().any(|name| name.starts_with('t')))
        .count()
        .to_string()
}

pub fn part2(input: &Graph) -> String {
    max_clique(
        input,
        HashSet::new(),
        input.keys().cloned().collect(),
        HashSet::new(),
    )
    .iter()
    .sorted()
    .join(",")
}

#[test]
fn test_day23() {
    let input = parse("ka-co\nta-co\nde-co\nta-ka\nde-ta\nka-de");
    assert_eq!(part1(&input), "3");
    assert_eq!(part2(&input), "co,de,ka,ta");
}
