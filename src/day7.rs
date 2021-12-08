use anyhow::Result;
use std::io::BufRead;

const DATA_FILE: &str = "7.txt";

pub fn data<P: AsRef<std::path::Path>>(data_dir: P) -> Result<Vec<u32>> {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let data = std::io::BufReader::new(std::fs::File::open(&data_file)?);
    let mut result: Vec<u32> = data
        .split(b',')
        .map(|b_res| {
            b_res
                .map_err(|e| e.into())
                .and_then(|b| String::from_utf8(b).map_err(|e| e.into()))
                .and_then(|s| s.trim().parse::<u32>().map_err(|e| e.into()))
        })
        .collect::<Result<Vec<u32>>>()?;
    result.sort_unstable();
    Ok(result)
}

fn abs_difference<T: std::ops::Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

// Fuel usage is linear - 1 point of fuel for each point of distance to target
fn score_solution_linear(data: &[u32], target: u32) -> u32 {
    let score = data.iter().map(|pos| abs_difference(*pos, target)).sum();
    score
}

// Fuel usage is triangular - sum(1..=n) for n points of distance to target
// formula is n*(n+1)/2
fn fuel_usage(dist: u32) -> u32 {
    (dist * (dist + 1)) / 2
}

fn score_solution_triangular(data: &[u32], target: u32) -> u32 {
    let score = data
        .iter()
        .map(|pos| fuel_usage(abs_difference(*pos, target)))
        .sum();
    score
}

fn optimize_score<F: Fn(u32) -> u32>(data: &[u32], scoring_function: F) -> u32 {
    let mut min_data: Vec<u32> = Vec::from(data);
    min_data.dedup();
    if let Some(window) = min_data.windows(3).find(|window| {
        scoring_function(window[0]) > scoring_function(window[1])
            && scoring_function(window[1]) < scoring_function(window[2])
    }) {
        (window[0]..=window[2]).map(scoring_function).min().unwrap()
    } else {
        panic!();
    }
}

pub fn star1(data: &[u32]) -> u32 {
    optimize_score(data, |target| score_solution_linear(data, target))
}

pub fn star2(data: &[u32]) -> u32 {
    optimize_score(data, |target| score_solution_triangular(data, target))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: [u32; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_star1() {
        let mut sample_data = Vec::from(SAMPLE_DATA);
        sample_data.sort_unstable();
        assert_eq!(star1(&sample_data), 37);
    }

    #[test]
    fn test_star2() {
        let mut sample_data = Vec::from(SAMPLE_DATA);
        sample_data.sort();
        assert_eq!(star2(&sample_data), 168);
    }
}
