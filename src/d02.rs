use std::error::Error;

// ABC 123
// XYZ 036
// A < B, B < C, C < A

pub fn solution1() -> Result<usize, Box<dyn Error>> {
    let lines = std::io::stdin().lines().collect::<Result<Vec<_>, _>>()?;

    Ok(lines.iter().map(|ln| compute_score(ln)).sum())
}

pub fn compute_score(ln: &str) -> usize {
    let (opp, me) = (ln.chars().next().unwrap(), ln.chars().nth(2).unwrap());

    let opp = opp as usize - 65;
    let me = me as usize - 88;

    1 + match me {
        _ if me == (opp + 1) % 3 => me + 6,
        _ if me == opp => me + 3,
        _ => me,
    }
}

// ABC opp R/P/C
// XYZ L/D/W
// A < B, B < C, C < A

pub fn solution2() -> Result<usize, Box<dyn Error>> {
    let lines = std::io::stdin().lines().collect::<Result<Vec<_>, _>>()?;

    Ok(lines.iter().map(|ln| compute_score_2(ln)).sum())
}

pub fn compute_score_2(ln: &str) -> usize {
    let (opp, play) = (ln.chars().next().unwrap(), ln.chars().nth(2).unwrap());

    let opp = opp as usize - 65;
    let play = play as usize - 88;

    match play {
        1 => 4 + opp,
        2 => 7 + (opp + 1) % 3,
        _ => 1 + (opp + 2) % 3,
    }
}
