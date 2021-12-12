use anyhow::Result;
use std::collections::HashSet;
use std::io::BufRead;

const DATA_FILE: &str = "11.txt";

pub fn data<P: AsRef<std::path::Path>>(data_dir: P) -> Result<Vec<Vec<u8>>> {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let data = std::io::BufReader::new(std::fs::File::open(&data_file)?);
    data.lines()
        .map(|b_res| {
            b_res
                .map(|s| s.bytes().map(|b| b - b'0').collect::<Vec<u8>>())
                .map_err(|e| e.into())
        })
        .collect::<Result<Vec<Vec<u8>>>>()
}

fn get_adjacents(data: &[Vec<u8>], point: &(usize, usize)) -> HashSet<(usize, usize)> {
    let (y, x) = point;
    let mut adjacents: HashSet<(usize, usize)> = HashSet::with_capacity(4);

    let min_y = y.saturating_sub(1);
    let min_x = x.saturating_sub(1);
    let max_y = (data.len() - 1).min(y + 1);
    let max_x = (data[max_y].len() - 1).min(x + 1);

    for new_y in min_y..=max_y {
        for new_x in min_x..=max_x {
            adjacents.insert((new_y, new_x));
        }
    }

    adjacents.remove(point);
    adjacents
}

fn step(grid: &mut Vec<Vec<u8>>) -> u32 {
    let mut flashed: HashSet<(usize, usize)> = HashSet::with_capacity(50);
    let mut last_flashed: HashSet<(usize, usize)> = HashSet::with_capacity(10);

    for (y, row) in grid.iter_mut().enumerate() {
        for (x, val) in row.iter_mut().enumerate() {
            *val += 1;
            if *val > 9 {
                last_flashed.insert((y, x));
                flashed.insert((y, x));
                *val = 0;
            }
        }
    }

    while !last_flashed.is_empty() {
        let mut just_flashed: HashSet<(usize, usize)> = HashSet::with_capacity(10);
        for point in last_flashed.drain() {
            let adjacents: Vec<(usize, usize)> = get_adjacents(grid, &point)
                .iter()
                .copied()
                .filter(|adj| !flashed.contains(adj))
                .collect();
            for adjacent_point in adjacents {
                let (y, x) = adjacent_point;
                grid[y][x] += 1;
                if grid[y][x] > 9 {
                    just_flashed.insert((y, x));
                    flashed.insert((y, x));
                    grid[y][x] = 0;
                }
            }
        }
        last_flashed.extend(just_flashed.drain());
    }
    flashed.len() as u32
}

pub fn star1(mut data: Vec<Vec<u8>>) -> u32 {
    let mut count = 0;
    for _ in 0..100 {
        let new_flashed = step(&mut data);
        count += new_flashed;
    }
    count
}

pub fn star2(mut data: Vec<Vec<u8>>) -> u32 {
    let size = (data.len() * data[0].len()) as u32;

    let mut days: u32 = 0;
    loop {
        days += 1;
        let flashed = step(&mut data);
        if flashed == size {
            return days;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: [[u8; 10]; 10] = [
        [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ];

    #[test]
    fn test_star1() {
        let sample_data: Vec<Vec<u8>> = SAMPLE_DATA.iter().map(|r| r.to_vec()).collect();
        assert_eq!(star1(sample_data), 1656);
    }

    #[test]
    fn test_star2() {
        let sample_data: Vec<Vec<u8>> = SAMPLE_DATA.iter().map(|r| r.to_vec()).collect();
        assert_eq!(star2(sample_data), 195);
    }
}
