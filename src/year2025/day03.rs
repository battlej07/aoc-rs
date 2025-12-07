pub const INPUT_PATH: &str = "input/2025/day03.txt";

pub fn part1(input: String) -> u32 {
    input
        .lines()
        .map(|bank| {
            let digits: Vec<u32> = bank
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect();

            let n = digits.len();
            let mut best = 0;

            for i in 0..n.saturating_sub(1) {
                for j in i + 1..n {
                    let val = digits[i] * 10 + digits[j];
                    if val > best {
                        best = val
                    }
                }
            }

            best
        })
        .sum()
}

pub fn part2(input: String) -> u64 {
    input
        .lines()
        .map(|bank| {
            let digites: Vec<u64> = bank
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect();

            let mut best = 0;
            let l = digites.len();

            let mut needed = 12;
            let mut position = 0;

            while needed > 0 {
                let max_pos = l - needed;

                let available_values = &digites[position..=max_pos];
                if let Some(best_value) = first_max(available_values) {
                    best = 10 * best + best_value.1;

                    needed -= 1;
                    position += best_value.0 + 1
                } else {
                    break;
                }
            }

            best
        })
        .sum()
}

fn first_max(arr: &[u64]) -> Option<(usize, u64)> {
    if arr.is_empty() {
        return None;
    }

    let mut max_index = 0;
    for (i, &v) in arr.iter().enumerate() {
        if v > arr[max_index] {
            max_index = i;
        }
    }

    Some((max_index, arr[max_index]))
}

#[cfg(test)]
mod test {
    use crate::year2025::day03::{part1, part2};

    #[test]
    fn part1_example() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#
            .to_string();

        assert_eq!(357, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#
            .to_string();

        assert_eq!(3121910778619, part2(input));
    }
}
