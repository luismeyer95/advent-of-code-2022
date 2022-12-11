use std::error::Error;
use std::{collections::HashSet, ops::Add};

use crate::utils;

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Fwd,
    Back,
    Still,
}

impl Add<Dir> for usize {
    type Output = usize;

    fn add(self, dir: Dir) -> Self::Output {
        match dir {
            Dir::Fwd => self + 1,
            Dir::Back => self.saturating_sub(1),
            Dir::Still => self,
        }
    }
}

pub fn solution1() -> Result<usize, Box<dyn Error>> {
    let lines = utils::parse_lines()?;
    let mat = parse_matrix(lines);
    let s = mat.len();

    let sets = vec![
        count_visible((Dir::Still, Dir::Fwd), (0..s).map(|n| (n, 0)), &mat),
        count_visible((Dir::Fwd, Dir::Still), (0..s).map(|n| (0, n)), &mat),
        count_visible((Dir::Still, Dir::Back), (0..s).map(|n| (n, s - 1)), &mat),
        count_visible((Dir::Back, Dir::Still), (0..s).map(|n| (s - 1, n)), &mat),
    ];

    let visible_count = sets
        .into_iter()
        .reduce(|acc, set| set.union(&acc).copied().collect())
        .unwrap()
        .len();

    Ok(visible_count)
}

pub fn parse_matrix<'a>(lines: Vec<String>) -> Vec<Vec<usize>> {
    lines
        .into_iter()
        .map(|ln| {
            ln.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

pub fn count_visible(
    (ys, xs): (Dir, Dir),
    edges: impl Iterator<Item = (usize, usize)>,
    mat: &Vec<Vec<usize>>,
) -> HashSet<(usize, usize)> {
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    let s = mat.len();

    for (mut y, mut x) in edges {
        let mut prev_top: Option<usize> = None;
        for _ in 0..s {
            let current = mat.get(y).and_then(|row| row.get(x)).unwrap();
            match prev_top {
                Some(top) if *current > top => {
                    prev_top = Some(*current);
                    visible.insert((y, x));
                }
                None => {
                    prev_top = Some(*current);
                    visible.insert((y, x));
                }
                _ => {}
            };
            y = y + ys;
            x = x + xs;
        }
    }

    visible
}
