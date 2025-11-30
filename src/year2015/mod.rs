use std::fs;

use crate::AocError;

mod day01;
mod day02;

pub fn run(day: u8, part: u8) -> Result<String, AocError> {
    match day {
        1 => match part {
            1 => Ok(day01::part1(read_file_to_string(day01::INPUT_PATH)).to_string()),
            2 => Ok(day01::part2(read_file_to_string(day01::INPUT_PATH)).to_string()),
            _ => Err(AocError::PartNotFound {
                year: 2015,
                day,
                part,
            }),
        },
        2 => match part {
            1 => Ok(day02::part1(read_file_to_string(day02::INPUT_PATH)).to_string()),
            2 => Ok(day02::part2(read_file_to_string(day02::INPUT_PATH)).to_string()),
            _ => Err(AocError::PartNotFound {
                year: 2015,
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
