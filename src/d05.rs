use crate::utils;
use recap::Recap;
use serde::Deserialize;
use std::{collections::VecDeque, error::Error};

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"move (?P<amount>\d+) from (?P<source>\d+) to (?P<destination>\d+)"#)]
pub struct MoveInstruction {
    amount: usize,
    source: usize,
    destination: usize,
}

pub fn solution1() -> Result<String, Box<dyn Error>> {
    solution(|moves, stacks| {
        for MoveInstruction {
            amount,
            source,
            destination,
        } in moves
        {
            for _ in 0..amount {
                if let Some(popped) = stacks[source].pop_back() {
                    stacks[destination].push_back(popped);
                }
            }
        }
    })
}

pub fn solution2() -> Result<String, Box<dyn Error>> {
    solution(|moves, stacks| {
        for MoveInstruction {
            amount,
            source,
            destination,
        } in moves
        {
            let src = &mut stacks[source];
            let slice: Vec<char> = src.drain((src.len() - amount)..).collect();
            stacks[destination].extend(slice);
        }
    })
}

pub fn solution(
    move_executor: impl Fn(Vec<MoveInstruction>, &mut [VecDeque<char>]),
) -> Result<String, Box<dyn Error>> {
    let input = utils::parse_lines()?;

    let mut lines = input
        .split(|ln| ln.is_empty() || ln.chars().next().is_some_and(|c| c.is_whitespace()))
        .filter(|slice| !slice.is_empty());

    let mut stacks = parse_stacks(lines.next().ok_or("bad input")?);
    let moves = parse_moves(lines.next().ok_or("bad input")?);

    move_executor(moves, &mut stacks);

    let result = stacks
        .into_iter()
        .skip(1)
        .filter_map(|s| s.back().copied())
        .collect::<String>();

    Ok(result)
}

pub fn parse_stacks(stacks_input: &[String]) -> Vec<VecDeque<char>> {
    let n_stacks = (stacks_input[0].len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::default(); n_stacks + 1];

    for ln in stacks_input {
        let crate_row = parse_row_of_crates(ln.as_ref());
        for (stack_number, letter) in crate_row {
            stacks[stack_number].push_front(letter);
        }
    }

    stacks
}

pub fn parse_row_of_crates(row: &str) -> Vec<(usize, char)> {
    row.chars()
        .enumerate()
        .filter_map(|(i, c)| {
            (i % 4 == 1 && c.is_alphabetic()).then_some((1 + (i.saturating_sub(1)) / 4, c))
        })
        .collect()
}

pub fn parse_moves(lines: impl IntoIterator<Item = impl AsRef<str>>) -> Vec<MoveInstruction> {
    lines
        .into_iter()
        .filter_map(|ln| ln.as_ref().parse::<MoveInstruction>().ok())
        .collect()
}
