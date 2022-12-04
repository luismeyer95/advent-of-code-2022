use std::collections::BinaryHeap;
use std::error::Error;

pub fn solution1() -> Result<usize, Box<dyn Error>> {
    let lines = std::io::stdin().lines().collect::<Result<Vec<_>, _>>()?;
    let max = lines
        .split(|s| s.is_empty())
        .map(|slice| slice.iter().map(|ln| ln.parse::<usize>().unwrap()).sum())
        .max();

    Ok(max.unwrap())
}

pub fn solution2() -> Result<isize, Box<dyn Error>> {
    let lines = std::io::stdin().lines().collect::<Result<Vec<_>, _>>()?;
    let calories_per_elve = lines.split(|s| s.is_empty()).map(|slice| {
        slice
            .iter()
            .map(|ln| ln.parse::<isize>().unwrap())
            .sum::<isize>()
    });

    let mut heap = BinaryHeap::<isize>::new();
    for cals in calories_per_elve {
        heap.push(-cals);
        if heap.len() > 3 {
            heap.pop();
        }
    }

    Ok(-heap.iter().sum::<isize>())
}
