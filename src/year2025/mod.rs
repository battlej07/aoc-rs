use std::fs;

use crate::AocError;

mod day01;
mod day02;
mod day03;
mod day04;

pub fn run(day: u8, part: u8) -> Result<String, AocError> {
    match day {
        1 => match part {
            1 => Ok(day01::part1(read_file_to_string(day01::INPUT_PATH)).to_string()),
            2 => Ok(day01::part2(read_file_to_string(day01::INPUT_PATH)).to_string()),
            _ => Err(AocError::PartNotFound {
                year: 2025,
                day,
                part,
            }),
        },
        2 => match part {
            1 => Ok(day02::part1(read_file_to_string(day02::INPUT_PATH)).to_string()),
            2 => Ok(day02::part2(read_file_to_string(day02::INPUT_PATH)).to_string()),
            _ => Err(AocError::PartNotFound {
                year: 2025,
                day,
                part,
            }),
        },
        3 => match part {
            1 => Ok(day03::part1(read_file_to_string(day03::INPUT_PATH)).to_string()),
            2 => Ok(day03::part2(read_file_to_string(day03::INPUT_PATH)).to_string()),
            _ => Err(AocError::PartNotFound {
                year: 2025,
                day,
                part,
            }),
        },
        4 => match part {
            1 => Ok(day04::part1(read_file_to_string(day04::INPUT_PATH)).to_string()),
            2 => Ok(day04::part2(read_file_to_string(day04::INPUT_PATH)).to_string()),
            _ => Err(AocError::PartNotFound {
                year: 2025,
                day,
                part,
            }),
        },
        _ => Err(AocError::DayNotFound { year: 2015, day }),
    }
}

fn read_file_to_string(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    }
}
