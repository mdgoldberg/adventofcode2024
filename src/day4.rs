use std::path::PathBuf;

use crate::common::read_file;

const XMAS: &str = "XMAS";
const MAS: &str = "MAS";

pub fn day4(path: PathBuf) {
    let input = read_file(path);
    let part1 = compute_part1(&input);
    println!("Part 1: {part1}");
    let part2 = compute_part2(&input);
    println!("Part 2: {part2}");
}

fn compute_part1(input: &str) -> usize {
    let wordsearch = WordSearch::new(input);
    let all_words = get_all_words(&wordsearch, XMAS.len());
    all_words.iter().filter(|&s| s == XMAS).count()
}

#[derive(Debug, Clone)]
struct X {
    word1: String,
    word2: String,
}

fn compute_part2(input: &str) -> usize {
    let wordsearch = WordSearch::new(input);
    let all_xs = get_all_xs(&wordsearch, MAS.len());
    all_xs
        .iter()
        .filter(|&x| x.word1 == MAS && x.word2 == MAS)
        .count()
}

struct WordSearch {
    lines: Vec<String>,
    width: usize,
    length: usize,
}

impl WordSearch {
    fn new(input: &str) -> Self {
        let lines: Vec<_> = input.lines().map(|s| s.to_string()).collect();
        let width = lines[0].len();
        let length = lines.len();
        Self {
            lines,
            width,
            length,
        }
    }

    fn get_down_word(&self, row_idx: usize, col_idx: usize, word_length: usize) -> Option<String> {
        let mut chars = Vec::new();
        for offset in 0..word_length {
            let row = row_idx + offset;
            chars.push(*self.lines.get(row)?.as_bytes().get(col_idx)?);
        }
        String::from_utf8(chars).ok()
    }

    fn get_right_word(&self, row_idx: usize, col_idx: usize, word_length: usize) -> Option<String> {
        let mut chars = Vec::new();
        for offset in 0..word_length {
            let col = col_idx + offset;
            chars.push(*self.lines.get(row_idx)?.as_bytes().get(col)?);
        }
        String::from_utf8(chars).ok()
    }

    fn get_downright_word(
        &self,
        row_idx: usize,
        col_idx: usize,
        word_length: usize,
    ) -> Option<String> {
        let mut chars = Vec::new();
        for offset in 0..word_length {
            let row = row_idx + offset;
            let col = col_idx + offset;
            chars.push(*self.lines.get(row)?.as_bytes().get(col)?);
        }
        String::from_utf8(chars).ok()
    }
    fn get_upright_word(
        &self,
        row_idx: usize,
        col_idx: usize,
        word_length: usize,
    ) -> Option<String> {
        let mut chars = Vec::new();
        for offset in 0..word_length {
            let row = row_idx.checked_sub(offset)?;
            let col = col_idx + offset;
            chars.push(*self.lines.get(row)?.as_bytes().get(col)?);
        }
        String::from_utf8(chars).ok()
    }
}

fn get_all_words(wordsearch: &WordSearch, word_length: usize) -> Vec<String> {
    let mut result = Vec::new();
    for row_idx in 0..wordsearch.length {
        for col_idx in 0..wordsearch.width {
            if let Some(word) = wordsearch.get_down_word(row_idx, col_idx, word_length) {
                let rev: String = word.chars().rev().collect();
                result.push(word);
                result.push(rev);
            }
            if let Some(word) = wordsearch.get_right_word(row_idx, col_idx, word_length) {
                let rev: String = word.chars().rev().collect();
                result.push(word);
                result.push(rev);
            }
            if let Some(word) = wordsearch.get_upright_word(row_idx, col_idx, word_length) {
                let rev: String = word.chars().rev().collect();
                result.push(word);
                result.push(rev);
            }
            if let Some(word) = wordsearch.get_downright_word(row_idx, col_idx, word_length) {
                let rev: String = word.chars().rev().collect();
                result.push(word);
                result.push(rev);
            }
        }
    }
    result
}

fn get_all_xs(wordsearch: &WordSearch, word_length: usize) -> Vec<X> {
    let mut result = Vec::new();
    for row_idx in 0..wordsearch.length {
        for col_idx in 0..wordsearch.width {
            if let Some(word1) = wordsearch.get_downright_word(row_idx, col_idx, word_length) {
                if let Some(word2) =
                    wordsearch.get_upright_word(row_idx + word_length / 2 + 1, col_idx, word_length)
                {
                    let word1_rev: String = word1.chars().rev().collect();
                    let word2_rev: String = word2.chars().rev().collect();
                    for word1 in &[word1.clone(), word1_rev.clone()] {
                        for word2 in &[word2.clone(), word2_rev.clone()] {
                            let x = X {
                                word1: word1.clone(),
                                word2: word2.clone(),
                            };
                            result.push(x);
                        }
                    }
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = r#"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
        "#
        .trim()
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n");
        assert_eq!(compute_part1(&input), 18);
    }
    #[test]
    fn part2_sample() {
        let input = r#"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
        "#
        .trim()
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n");
        assert_eq!(compute_part2(&input), 9);
    }
}
