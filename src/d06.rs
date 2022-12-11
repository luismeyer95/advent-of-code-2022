use crate::utils;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

pub fn solution1() -> Result<usize, Box<dyn Error>> {
    let datastream: Vec<char> = utils::parse_line()?.chars().collect();
    solution(datastream, 4)
}

pub fn solution2() -> Result<usize, Box<dyn Error>> {
    let datastream: Vec<char> = utils::parse_line()?.chars().collect();
    solution(datastream, 14)
}

pub fn solution(datastream: Vec<char>, k: usize) -> Result<usize, Box<dyn Error>> {
    for (i, win) in datastream.windows(k).enumerate() {
        if win.iter().copied().collect::<HashSet<char>>().len() == k {
            return Ok(i + k);
        }
    }

    Err("not found".into())
}

pub fn solution1_alt() -> Result<usize, Box<dyn Error>> {
    let datastream: Vec<char> = utils::parse_line()?.chars().collect();
    let mut counter = HashMap::<char, usize>::new();
    let mut l = 0;

    for r in 0..datastream.len() {
        if r >= 4 {
            counter.entry(datastream[l]).and_modify(|count| *count -= 1);
            if *counter.get(&datastream[l]).unwrap() == 0 {
                counter.remove(&datastream[l]);
            }
            l += 1;
        }
        counter
            .entry(datastream[r])
            .and_modify(|count| *count += 1)
            .or_insert(1);
        if counter.len() == 4 && counter.values().all(|&c| c == 1) {
            return Ok(r + 1);
        }
    }

    Err("not found".into())
}
