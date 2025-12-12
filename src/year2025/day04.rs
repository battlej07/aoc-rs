use std::{collections::HashMap, isize, usize};

pub const INPUT_PATH: &str = "input/2025/day04.txt";

pub fn part1(input: String) -> u64 {
    let mut position_is_free: HashMap<(usize, usize), bool> = HashMap::new();

    for (line_number, line) in input.lines().enumerate() {
        for (pos, c) in line.char_indices() {
            position_is_free.insert((line_number, pos), c == '.');
        }
    }

    todo!()
}

pub fn part2(input: String) -> u64 {
    todo!()
}

fn top_left(line: usize, pos: usize) -> (usize, usize) {
    (line - 1, pos - 1)
}

fn top_center(line: usize, pos: usize) -> (usize, usize) {
    (line - 1, pos)
}

fn top_right(line: usize, pos: usize) -> (usize, usize) {
    (line - 1, pos + 1)
}

fn center_left(line: usize, pos: usize) -> (usize, usize) {
    (line, pos - 1)
}

fn center_right(line: usize, pos: usize) -> (usize, usize) {
    (line, pos + 1)
}

fn bottom_left(line: usize, pos: usize) -> (usize, usize) {
    (line + 1, pos - 1)
}

fn bottom_center(line: usize, pos: usize) -> (usize, usize) {
    (line + 1, pos)
}

fn bottom_right(line: usize, pos: usize) -> (usize, usize) {
    (line + 1, pos + 1)
}

#[cfg(test)]
mod test {
    use crate::year2025::day04::part1;

    #[test]
    fn part1_example() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#
            .to_string();

        assert_eq!(13, part1(input));
    }

    #[test]
    fn part2_example() {
        todo!();
    }
}
