pub const INPUT_PATH: &str = "input/2015/day01.txt";

pub fn part1(input: String) -> i64 {
    let mut level = 0;

    for c in input.chars() {
        if c == '(' {
            level += 1;
        } else if c == ')' {
            level -= 1;
        }
    }

    level
}

pub fn part2(input: String) -> i64 {
    let mut level = 0;

    for (i, c) in input.char_indices() {
        if c == '(' {
            level += 1;
        } else if c == ')' {
            level -= 1
        }

        if level == -1 {
            return i as i64 + 1;
        }
    }

    0
}

#[cfg(test)]
mod test {
    use crate::year2015::day01::{part1, part2};

    #[test]
    fn part1_examples() {
        assert_eq!(0, part1("(())".to_string()));
        assert_eq!(0, part1("()()".to_string()));
        assert_eq!(3, part1("(((".to_string()));
        assert_eq!(3, part1("(()(()(".to_string()));
        assert_eq!(3, part1("))(((((".to_string()));
        assert_eq!(-1, part1("())".to_string()));
        assert_eq!(-1, part1("))(".to_string()));
        assert_eq!(-3, part1(")))".to_string()));
        assert_eq!(-3, part1(")())())".to_string()));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(1, part2(")".to_string()));
        assert_eq!(5, part2("()())".to_string()));
    }
}
