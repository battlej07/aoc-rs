pub const INPUT_PATH: &str = "input/2025/day01.txt";

struct Dial {
    position: i32,
}

impl Dial {
    fn new() -> Self {
        Dial { position: 50 }
    }

    fn turn_right(&mut self, amount: i32) -> i32 {
        self.position = (self.position + amount).rem_euclid(100);
        self.position
    }

    fn turn_left(&mut self, amount: i32) -> i32 {
        self.position = (self.position - amount).rem_euclid(100);
        self.position
    }
}

pub fn part1(input: String) -> i32 {
    let mut dial = Dial::new();
    let mut code = 0;

    for line in input.lines() {
        let (direction, amount) = line.split_at(1);

        let x = match direction {
            "R" => dial.turn_right(amount.parse().unwrap()),
            "L" => dial.turn_left(amount.parse().unwrap()),
            _ => panic!("Parsing didn't work"),
        };

        if x == 0 {
            code += 1
        }
    }

    code
}

pub fn part2(input: String) -> i64 {
    let mut dial = Dial::new();
    let mut code = 0;

    for line in input.lines() {
        let (direction, amount) = line.split_at(1);

        code += match direction {
            "R" => {
                let mut zero_counter = 0;

                for _ in 1..=amount.parse().unwrap() {
                    let current_dial_position = dial.turn_right(1);

                    if current_dial_position == 0 {
                        zero_counter += 1;
                    }
                }

                zero_counter
            }
            "L" => {
                let mut zero_counter = 0;

                for _ in 1..=amount.parse().unwrap() {
                    let current_dial_position = dial.turn_left(1);

                    if current_dial_position == 0 {
                        zero_counter += 1;
                    }
                }

                zero_counter
            }
            _ => panic!("Parsing didn't work"),
        };
    }

    code
}

#[cfg(test)]
mod test {
    use crate::year2025::day01::{part1, part2};

    #[test]
    fn part1_example() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#
            .to_string();

        assert_eq!(3, part1(input))
    }

    #[test]
    fn part2_example() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#
            .to_string();

        assert_eq!(6, part2(input));
    }
}
