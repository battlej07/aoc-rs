pub const INPUT_PATH: &str = "input/2025/day02.txt";

pub fn part1(input: String) -> u64 {
    let ranges = input.split(",").map(parse_range);

    let mut sum = 0;

    let is_repeating = |range: &str| {
        let len = range.len();

        if !len.is_multiple_of(2) {
            return false;
        }

        let half = len / 2;

        range[0..half] == range[half..len]
    };

    for (lower, upper) in ranges {
        for i in lower..=upper {
            if is_repeating(&i.to_string()) {
                sum += i;
            }
        }
    }

    sum
}

pub fn part2(input: String) -> u32 {
    todo!()
}

fn parse_range(range_str: &str) -> (u64, u64) {
    let (lower, upper) = range_str.split_once("-").unwrap();

    (
        lower.trim().parse().unwrap_or_default(),
        upper.trim().parse().unwrap_or_default(),
    )
}

#[cfg(test)]
mod test {
    use crate::year2025::day02::{parse_range, part1};

    #[test]
    fn parse_range_test() {
        assert_eq!((5, 23), parse_range("5-23"));
        assert_eq!((2284, 93939), parse_range("2284-93939"));
    }

    #[test]
    fn part1_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();

        assert_eq!(1227775554, part1(input));
    }

    #[test]
    fn part2_example() {
        todo!();
    }
}
