use std::{collections::HashSet, path::PathBuf};

use crate::common::read_file;

pub fn day6(path: PathBuf) {
    let input = read_file(path);
    let part1 = compute_part1(&input);
    println!("Part 1: {part1}");
    let part2 = compute_part2(&input);
    println!("Part 2: {part2}");
}

fn compute_part1(input: &str) -> usize {
    let mut map_state = parse_input(input);
    let mut visited = HashSet::from([map_state.cur_location]);
    while let Some(cur_state) = map_state.next_state() {
        map_state = cur_state;
        visited.insert(map_state.cur_location);
    }
    visited.len()
}

fn compute_part2(input: &str) -> usize {
    let starting_state = parse_input(input);
    let initial_location = starting_state.cur_location;
    let width = starting_state.map[0].len();
    let length = starting_state.map.len();
    let mut blockers = HashSet::new();
    // brute forcing!
    // smarter idea: keep track of obstacles hit and direction they're hit in, and check if adding
    // an obstacle would lead the guard to hit those obstacles
    for row_idx in 0..length {
        for col_idx in 0..width {
            let location = (row_idx, col_idx);
            if location == initial_location {
                continue;
            }
            let new_state = starting_state.with_obstacle(&location);
            if new_state.leads_to_loop() {
                blockers.insert(location);
            }
        }
    }
    blockers.len()
}

#[derive(Debug, Clone)]
struct MapState {
    map: Vec<Vec<Square>>,
    cur_location: (usize, usize),
    cur_direction: Direction,
}

#[derive(Debug, Copy, Clone)]
enum Square {
    Free,
    Obstacle,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn neighboring_square(location: (usize, usize), direction: Direction) -> (isize, isize) {
    let location = (location.0 as isize, location.1 as isize);
    match direction {
        Direction::Up => (location.0 - 1, location.1),
        Direction::Down => (location.0 + 1, location.1),
        Direction::Left => (location.0, location.1 - 1),
        Direction::Right => (location.0, location.1 + 1),
    }
}

impl MapState {
    fn get_square(&self, location: &(usize, usize)) -> Square {
        self.map[location.0][location.1]
    }

    /// None means the guard has exited the map
    fn next_state(self) -> Option<Self> {
        let potential_next_location = neighboring_square(self.cur_location, self.cur_direction);
        if potential_next_location.0 >= self.map.len() as isize
            || potential_next_location.0 < 0
            || potential_next_location.1 >= self.map[0].len() as isize
            || potential_next_location.1 < 0
        {
            return None;
        }
        let potential_next_location = (
            potential_next_location.0 as usize,
            potential_next_location.1 as usize,
        );
        let potential_next_square = self.get_square(&potential_next_location);
        let (new_location, new_direction) = match potential_next_square {
            Square::Free => (potential_next_location, self.cur_direction),
            Square::Obstacle => (self.cur_location, self.cur_direction.next_direction()),
        };
        Some(Self {
            map: self.map,
            cur_location: new_location,
            cur_direction: new_direction,
        })
    }

    fn with_obstacle(&self, location: &(usize, usize)) -> Self {
        let Self {
            cur_location,
            cur_direction,
            ..
        } = *self;
        let mut new_map = self.map.clone();
        new_map[location.0][location.1] = Square::Obstacle;
        Self {
            map: new_map,
            cur_location,
            cur_direction,
        }
    }

    fn leads_to_loop(&self) -> bool {
        let mut visited = HashSet::from([(self.cur_location, self.cur_direction)]);
        let mut cur_state = (*self).clone();
        while let Some(next_state) = cur_state.next_state() {
            cur_state = next_state;
            if !visited.insert((cur_state.cur_location, cur_state.cur_direction)) {
                return true;
            }
        }
        false
    }
}

impl From<char> for Square {
    fn from(value: char) -> Self {
        match value {
            '.' | '^' => Self::Free,
            '#' => Self::Obstacle,
            _ => panic!("Invalid character: {value}"),
        }
    }
}

impl Direction {
    fn next_direction(&self) -> Self {
        match self {
            Direction::Up => Self::Right,
            Direction::Right => Self::Down,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
        }
    }
}

fn parse_input(input: &str) -> MapState {
    let lines = input.trim().lines();
    let width = lines.clone().next().unwrap().len();
    let length = lines.clone().count();
    let mut map = Vec::with_capacity(length);
    let mut location = None;
    for (row_idx, line) in input.trim().lines().enumerate() {
        let mut row = Vec::with_capacity(width);
        for (col_idx, char) in line.chars().enumerate() {
            let square = Square::from(char);
            row.push(square);
            if char == '^' {
                location = Some((row_idx, col_idx));
            }
        }
        map.push(row);
    }
    MapState {
        map,
        cur_location: location.unwrap(),
        cur_direction: Direction::Up,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = read_file("test_data/day6-sample1.txt".into());
        assert_eq!(compute_part1(&input), 41);
    }
    #[test]
    fn part2_sample() {
        let input = read_file("test_data/day6-sample1.txt".into());
        assert_eq!(compute_part2(&input), 6);
    }
}
