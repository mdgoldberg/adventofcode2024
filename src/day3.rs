use std::path::PathBuf;

use regex::Regex;

use crate::common::read_file;

pub fn day3(path: PathBuf) {
    let input = read_file(path);
    let part1 = compute_part1(&input);
    println!("{part1}");
    let part2 = compute_part2(&input);
    println!("{part2}");
}

fn parse_data_part1(input: &str) -> Vec<(f64, f64)> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    regex
        .captures_iter(input)
        .map(|c| {
            (
                c.get(1).unwrap().as_str().parse().unwrap(),
                c.get(2).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect()
}

fn compute_part1(input: &str) -> f64 {
    let parsed_data = parse_data_part1(input);
    parsed_data.iter().map(|(a, b)| a * b).sum()
}

enum Event {
    Toggle { enabled: bool },
    Mul { a: f64, b: f64 },
}

fn parse_data_part2(input: &str) -> Vec<Event> {
    let regex =
        Regex::new(r"(?<mul>mul\((?<a>\d+),(?<b>\d+)\))|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
    regex
        .captures_iter(input)
        .map(|c| {
            if c.name("mul").is_some() {
                Event::Mul {
                    a: c.name("a").unwrap().as_str().parse().unwrap(),
                    b: c.name("b").unwrap().as_str().parse().unwrap(),
                }
            } else if c.name("do").is_some() {
                Event::Toggle { enabled: true }
            } else {
                Event::Toggle { enabled: false }
            }
        })
        .collect()
}

fn compute_part2(input: &str) -> f64 {
    let events = parse_data_part2(input);
    let mut is_enabled = true;
    let mut sum = 0.0;
    for event in &events {
        match *event {
            Event::Toggle { enabled, .. } => {
                is_enabled = enabled;
            }
            Event::Mul { a, b, .. } => {
                if is_enabled {
                    sum += a * b;
                }
            }
        }
    }
    sum
}
