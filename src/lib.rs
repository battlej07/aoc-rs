use core::fmt;
use std::error::Error;

mod year2015;

#[derive(Debug, Clone)]
pub enum AocError {
    YearNotFound { year: u16 },
    DayNotFound { year: u16, day: u8 },
    PartNotFound { year: u16, day: u8, part: u8 },
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::YearNotFound { year } => write!(f, "Year {year} not found"),
            Self::DayNotFound { year, day } => write!(f, "Day {day} not found in year {year}"),
            Self::PartNotFound { year, day, part } => {
                write!(f, "Part {part} not found in day {day} of year {year}")
            }
        }
    }
}

impl Error for AocError {}

pub fn run(year: u16, day: u8, part: u8) -> Result<String, AocError> {
    match year {
        2015 => year2015::run(day, part),
        other => Err(AocError::YearNotFound { year: other }),
    }
}
