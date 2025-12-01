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

    println!("The dial starts by pointing at {}", dial.position);

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

#[allow(unused_variables)]
pub fn part2(input: String) -> i64 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::year2025::day01::part1;

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
}
