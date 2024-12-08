use std::path::PathBuf;

use petgraph::{algo::toposort, graphmap::DiGraphMap};
use regex::Regex;

use crate::common::read_file;

pub fn day5(path: PathBuf) {
    let input = read_file(path);
    let part1 = compute_part1(&input);
    println!("Part 1: {part1}");
    let part2 = compute_part2(&input);
    println!("Part 2: {part2}");
}

#[derive(Debug)]
struct ParsedData {
    pub rules: Vec<Rule>,
    pub queries: Vec<Query>,
}

#[derive(Debug)]
struct Rule {
    pub earlier: u32,
    pub later: u32,
}

type Query = Vec<u32>;

fn parse_input(input: &str) -> ParsedData {
    let regex = Regex::new(r"(?<earlier>\d+)\|(?<later>\d+)").unwrap();
    let rules = input
        .lines()
        .filter_map(|line| {
            let cap = regex.captures(line)?;
            Some(Rule {
                earlier: cap.name("earlier")?.as_str().parse().unwrap(),
                later: cap.name("later")?.as_str().parse().unwrap(),
            })
        })
        .collect();
    let queries = input
        .lines()
        .filter(|line| !line.contains('|') && !line.trim().is_empty())
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    ParsedData { rules, queries }
}

fn compute_part1(input: &str) -> u32 {
    let parsed = parse_input(input);
    parsed
        .queries
        .iter()
        .map(|query| {
            let graph_for_query: DiGraphMap<u32, ()> = parsed
                .rules
                .iter()
                .filter_map(|r| {
                    (query.contains(&r.earlier) && query.contains(&r.later))
                        .then_some((r.earlier, r.later))
                })
                .collect();
            let Ok(sorted) = toposort(&graph_for_query, None) else {
                return 0;
            };
            if sorted
                .iter()
                .map(|val| query.iter().position(|x| x == val).unwrap())
                .is_sorted()
            {
                sorted[sorted.len() / 2]
            } else {
                0
            }
        })
        .sum()
}
fn compute_part2(input: &str) -> u32 {
    let parsed = parse_input(input);
    parsed
        .queries
        .iter()
        .map(|query| {
            let graph_for_query: DiGraphMap<u32, ()> = parsed
                .rules
                .iter()
                .filter_map(|r| {
                    (query.contains(&r.earlier) && query.contains(&r.later))
                        .then_some((r.earlier, r.later))
                })
                .collect();
            let sorted = toposort(&graph_for_query, None).unwrap();
            if sorted
                .iter()
                .map(|val| query.iter().position(|x| x == val).unwrap())
                .is_sorted()
            {
                0
            } else {
                sorted[sorted.len() / 2]
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = read_file("test_data/day5-sample1.txt".into());
        assert_eq!(compute_part1(&input), 143);
    }
    #[test]
    fn part2_sample() {
        let input = read_file("test_data/day5-sample1.txt".into());
        assert_eq!(compute_part2(&input), 123);
    }
}
