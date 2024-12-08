use std::path::PathBuf;

use crate::common::read_file;

type ParsedData = Vec<Report>;
type Report = Vec<i32>;

fn parse_data(input: &str) -> ParsedData {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn is_safe(report: &[i32]) -> bool {
    let diffs: Vec<_> = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(a, b)| a - b)
        .collect();
    let increasing = diffs[0] > 0;
    diffs
        .iter()
        .all(|d| d.abs() >= 1 && d.abs() <= 3 && ((*d > 0) == increasing))
}

fn is_safe_part2(report: &[i32]) -> bool {
    let mut reports_to_check: Vec<Report> = vec![report.to_vec()];
    for i in 0..report.len() {
        let mut new_report = report.to_vec();
        new_report.remove(i);
        reports_to_check.push(new_report);
    }
    reports_to_check.iter().any(|report| is_safe(report))
}

fn compute_part1(data: ParsedData) -> usize {
    data.iter().filter(|x| is_safe(x)).count()
}

fn compute_part2(data: ParsedData) -> usize {
    data.iter().filter(|x| is_safe_part2(x)).count()
}

pub fn day2(path: PathBuf) {
    let input = read_file(path);
    let data = parse_data(&input);
    let part1 = compute_part1(data.clone());
    println!("{part1}");
    let part2 = compute_part2(data);
    println!("{part2}");
}
