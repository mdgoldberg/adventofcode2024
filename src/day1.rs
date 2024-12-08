use std::{collections::HashMap, path::PathBuf};

use regex::Regex;

use crate::common::read_file;

type ParsedData = (Vec<i64>, Vec<i64>);

fn parse_data(input: &str) -> ParsedData {
    let regex = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    input
        .lines()
        .filter_map(|l| regex.captures(l))
        .map(|cap| {
            (
                cap[1].parse::<i64>().unwrap(),
                cap[2].parse::<i64>().unwrap(),
            )
        })
        .unzip()
}

fn get_part1_result((mut list1, mut list2): ParsedData) -> u64 {
    list1.sort();
    list2.sort();
    list1.iter().zip(list2).map(|(a, b)| a.abs_diff(b)).sum()
}

fn get_part2_result((list1, list2): ParsedData) -> i64 {
    let mut counts = HashMap::new();
    list2.iter().for_each(|val| {
        counts.entry(val).and_modify(|x| *x += 1).or_insert(1);
    });
    list1
        .iter()
        .map(|val| counts.get(val).map(|x| x * val).unwrap_or(0))
        .sum()
}

pub fn day1(input_path: PathBuf) {
    let data = parse_data(&read_file(input_path));
    let part1 = get_part1_result(data.clone());
    println!("{}", part1);
    let part2 = get_part2_result(data);
    println!("{}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_file() -> String {
        r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        "#
        .to_string()
    }

    fn example_data() -> ParsedData {
        (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
    }

    #[test]
    fn test_parse_data() {
        let input = example_file();
        let lists = parse_data(&input);
        let example = example_data();
        assert_eq!(lists, example);
    }

    #[test]
    fn test_get_result() {
        let example = example_data();
        let result = get_part1_result(example);
        assert_eq!(result, 11);
    }
}
