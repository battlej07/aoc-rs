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

#[allow(unused_variables)]
pub fn part2(input: String) -> i64 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::year2015::day01::part1;

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
}
