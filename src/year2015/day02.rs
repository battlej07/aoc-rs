pub const INPUT_PATH: &str = "input/2015/day02.txt";

pub fn part1(input: String) -> i64 {
    let mut paper_needed = 0;

    for line in input.lines() {
        let split_line: Vec<&str> = line.split("x").collect();

        let l: i64 = split_line[0].parse().unwrap();
        let w: i64 = split_line[1].parse().unwrap();
        let h: i64 = split_line[2].parse().unwrap();

        let side_a = l * w;
        let side_b = w * h;
        let side_c = h * l;

        let smallest = [side_a, side_b, side_c].into_iter().min().unwrap();

        paper_needed += (2 * side_a + 2 * side_b + 2 * side_c) + smallest;
    }

    paper_needed
}

pub fn part2(input: String) -> i64 {
    let mut ribbon_needed = 0;

    for line in input.lines() {
        let split_line: Vec<&str> = line.split("x").collect();

        let l: i64 = split_line[0].parse().unwrap();
        let w: i64 = split_line[1].parse().unwrap();
        let h: i64 = split_line[2].parse().unwrap();

        let mut sorted = [l, w, h];

        sorted.sort();

        let wrap_ribbon = 2 * sorted[0] + 2 * sorted[1];

        let bow_ribbon = l * w * h;

        ribbon_needed += bow_ribbon + wrap_ribbon;
    }

    ribbon_needed
}

#[cfg(test)]
mod test {
    use crate::year2015::day02::{part1, part2};

    #[test]
    fn part1_examples() {
        assert_eq!(58, part1("2x3x4".to_string()));
        assert_eq!(43, part1("1x1x10".to_string()));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(34, part2("2x3x4".to_string()));
        assert_eq!(14, part2("1x1x10".to_string()));
    }
}
