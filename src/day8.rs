use std::path::PathBuf;

use crate::common::read_file;

pub fn day8(path: PathBuf) {
    let input = read_file(path);
    let part1 = compute_part1(&input);
    println!("Part 1: {part1}");
    let part2 = compute_part2(&input);
    println!("Part 2: {part2}");
}

fn compute_part1(_input: &str) -> usize {
    todo!()
}
fn compute_part2(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    // use super::*;
    //
    // #[test]
    // fn part1_sample() {
    //     let input = read_file("test_data/day8-sample1.txt".into());
    //     assert_eq!(compute_part1(&input), 41);
    // }
    // #[test]
    // fn part2_sample() {
    //     let input = read_file("test_data/day8-sample1.txt".into());
    //     assert_eq!(compute_part2(&input), 123);
    // }
}
