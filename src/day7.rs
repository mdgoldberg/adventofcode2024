use std::path::PathBuf;

use crate::common::read_file;

pub fn day7(path: PathBuf) {
    let input = read_file(path);
    let part1 = compute_part1(&input);
    println!("Part 1: {part1}");
    let part2 = compute_part2(&input);
    println!("Part 2: {part2}");
}

fn compute_part1(input: &str) -> i64 {
    let equations = parse_data(input);
    equations
        .iter()
        .filter_map(|eq| eq.is_solvable_part1().then_some(eq.target))
        .sum()
}
fn compute_part2(input: &str) -> i64 {
    let equations = parse_data(input);
    equations
        .iter()
        .filter_map(|eq| eq.is_solvable_part2().then_some(eq.target))
        .sum()
}

#[derive(Debug)]
struct Equation {
    target: i64,

    /// Stored in reverse order so that we can pop the current one
    operands: Vec<i64>,
}

impl Equation {
    fn is_solvable_part1(&self) -> bool {
        if self.operands.len() == 1 {
            return self.target == self.operands[0];
        } else if self.operands.len() == 2 {
            return (self.target == self.operands[0] + self.operands[1])
                || (self.target == self.operands[0] * self.operands[1]);
        }

        let mut add_operands = self.operands.clone();
        let new_number = add_operands.pop().unwrap();
        let new_last = add_operands.pop().unwrap();
        add_operands.push(new_number + new_last);
        let add_equation = Equation {
            target: self.target,
            operands: add_operands,
        };

        let mut mult_operands = self.operands.clone();
        let new_number = mult_operands.pop().unwrap();
        let new_last = mult_operands.pop().unwrap();
        mult_operands.push(new_number * new_last);
        let mult_equation = Equation {
            target: self.target,
            operands: mult_operands,
        };
        add_equation.is_solvable_part1() || mult_equation.is_solvable_part1()
    }

    fn is_solvable_part2(&self) -> bool {
        if self.operands.len() == 1 {
            return self.target == self.operands[0];
        } else if self.operands.len() == 2 {
            return (self.target == self.operands[0] + self.operands[1])
                || (self.target == self.operands[0] * self.operands[1])
                || (self.target.to_string()
                    == (self.operands[1].to_string() + &self.operands[0].to_string()));
        }

        let mut add_operands = self.operands.clone();
        let new_number = add_operands.pop().unwrap();
        let new_last = add_operands.pop().unwrap();
        add_operands.push(new_number + new_last);
        let add_equation = Equation {
            target: self.target,
            operands: add_operands,
        };

        let mut mult_operands = self.operands.clone();
        let new_number = mult_operands.pop().unwrap();
        let new_last = mult_operands.pop().unwrap();
        mult_operands.push(new_number * new_last);
        let mult_equation = Equation {
            target: self.target,
            operands: mult_operands,
        };

        let mut concat_operands = self.operands.clone();
        let new_number = concat_operands.pop().unwrap();
        let new_last = concat_operands.pop().unwrap();
        concat_operands.push(
            (new_number.to_string() + &new_last.to_string())
                .parse()
                .unwrap(),
        );
        let concat_equation = Equation {
            target: self.target,
            operands: concat_operands,
        };
        add_equation.is_solvable_part2()
            || mult_equation.is_solvable_part2()
            || concat_equation.is_solvable_part2()
    }
}

fn parse_data(input: &str) -> Vec<Equation> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (target_str, operands_str) = line.split_once(':').unwrap();
            let target = target_str.parse().unwrap();
            let operands = operands_str
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .rev()
                .collect();
            Equation { target, operands }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = read_file("test_data/day7-sample1.txt".into());
        assert_eq!(compute_part1(&input), 3749);
    }
    #[test]
    fn part2_sample() {
        let input = read_file("test_data/day7-sample1.txt".into());
        assert_eq!(compute_part2(&input), 11387);
    }
}
