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

    // let mut sum = 0;
    //
    // for bank in banks {
    //     let mut x: Vec<(usize, u32)> = bank
    //         .trim()
    //         .char_indices()
    //         .map(|(i, c)| (i, c.to_string().parse::<u32>().unwrap()))
    //         .collect();
    //
    //     x.sort_by(|a, b| a.1.cmp(&b.1));
    //
    //     let a = x[x.len() - 1];
    //     let b = x[x.len() - 2];
    //
    //     println!("Bank: {bank}");
    //     println!("Sorted: {:?}", x);
    //     println!("A: {:?}; B: {:?}", a, b);
    //
    //     sum += if a.0 < b.0 {
    //         format!("{}{}", a.1, b.1).parse::<u32>().unwrap()
    //     } else {
    //         format!("{}{}", b.1, a.1).parse::<u32>().unwrap()
    //     }
    // }
    //
    // sum
}

pub fn part2(input: String) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::year2025::day03::part1;

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
        todo!();
    }
}
