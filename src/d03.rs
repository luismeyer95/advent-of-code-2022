use std::{collections::HashSet, error::Error};

pub fn solution1() -> Result<usize, Box<dyn Error>> {
    let lines = std::io::stdin().lines().collect::<Result<Vec<_>, _>>()?;
    let mut sum = 0;

    for ln in lines {
        let c1: HashSet<char> = ln[..ln.len() / 2].chars().collect();
        let c2: HashSet<char> = ln[ln.len() / 2..].chars().collect();
        let common = c1.intersection(&c2).next().unwrap();
        sum += to_priority(*common);
    }

    Ok(sum)
}

pub fn solution() -> Result<usize, Box<dyn Error>> {
    let lines = std::io::stdin().lines().collect::<Result<Vec<_>, _>>()?;

    let priorities = lines.chunks(3).map(|lns| {
        let chr = lns
            .iter()
            .map(|ln| ln.chars().collect::<HashSet<char>>())
            .reduce(|acc, set| acc.intersection(&set).copied().collect())
            .and_then(|set| set.into_iter().next())
            .unwrap();

        if chr.is_lowercase() {
            chr as usize - 97 + 1
        } else {
            chr as usize - 65 + 27
        }
    });

    Ok(priorities.sum())
}

pub fn to_priority(ch: char) -> usize {
    if ch.is_lowercase() {
        ch as usize - 97 + 1
    } else {
        ch as usize - 65 + 27
    }
}
