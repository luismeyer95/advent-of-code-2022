use recap::Recap;
use serde::Deserialize;
use std::{
    cmp::{max, min},
    error::Error,
};

use crate::utils;

#[derive(Debug, Deserialize, Recap)]
#[recap(
    regex = r#"(?P<range_1_start>\d+)-(?P<range_1_end>\d+),(?P<range_2_start>\d+)-(?P<range_2_end>\d+)"#
)]
struct Input {
    range_1_start: usize,
    range_1_end: usize,
    range_2_start: usize,
    range_2_end: usize,
}

pub fn solution1() -> Result<usize, Box<dyn Error>> {
    let lines = utils::parse_lines()?;
    let mut total: usize = 0;

    for ln in lines {
        let input: Input = ln.parse()?;

        if input.range_1_start <= input.range_2_start && input.range_1_end >= input.range_2_end
            || input.range_2_start <= input.range_1_start && input.range_2_end >= input.range_1_end
        {
            total += 1
        }
    }

    Ok(total)
}

pub fn solution2() -> Result<usize, Box<dyn Error>> {
    let lines = utils::parse_lines()?;
    let mut total: usize = 0;

    for ln in lines {
        let input: Input = ln.parse()?;

        let start = max(input.range_1_start, input.range_2_start);
        let end = min(input.range_1_end, input.range_2_end);

        if start <= end {
            total += 1;
        }
    }

    Ok(total)
}
