use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

const DATA_FILE: &str = "9.txt";

pub fn data<P: AsRef<std::path::Path>>(data_dir: P) -> Result<Vec<Vec<u8>>> {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let data = std::io::BufReader::new(std::fs::File::open(&data_file)?);
    let result = data
        .lines()
        .map(|b_res| {
            b_res
                .map(|s| s.bytes().map(|b| b - b'0').collect::<Vec<u8>>())
                .map_err(|e| e.into())
        })
        .collect::<Result<Vec<Vec<u8>>>>()?;
    Ok(result)
}

fn get_adjacents(data: &[&[u8]], point: &(usize, usize)) -> HashSet<(usize, usize)> {
    let (y, x) = point;
    let mut adjacents: HashSet<(usize, usize)> = HashSet::with_capacity(4);

    if let Some(new_x) = x.checked_sub(1) {
        adjacents.insert((*y, new_x));
    }
    if data.get(*y).and_then(|r| r.get(x + 1)).is_some() {
        adjacents.insert((*y, x + 1));
    }
    if let Some(new_y) = y.checked_sub(1) {
        adjacents.insert((new_y, *x));
    }
    if data.get(y + 1).and_then(|r| r.get(*x)).is_some() {
        adjacents.insert((y + 1, *x));
    }
    adjacents
}

fn low_points(data: &[&[u8]]) -> HashMap<(usize, usize), u8> {
    let mut points: HashMap<(usize, usize), u8> = HashMap::new();
    for (y, row) in data.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if get_adjacents(data, &(y, x))
                .iter()
                .map(|(b, a)| data[*b][*a])
                .all(|other| val < &other)
            {
                points.insert((y, x), *val);
            }
        }
    }
    points
}

fn grow_basin(data: &[&[u8]], point: &(usize, usize), basin: &mut HashSet<(usize, usize)>) {
    let (y, x) = point;
    if data[*y][*x] >= 9 {
        return;
    } else {
        basin.insert(*point);
    }

    for point in get_adjacents(data, &(*y, *x)).iter() {
        if !basin.contains(point) {
            grow_basin(data, point, basin);
        }
    }
}

pub fn star1(data: &[&[u8]]) -> u32 {
    low_points(data).values().map(|d| (d + 1) as u32).sum()
}

pub fn star2(data: &[&[u8]]) -> u32 {
    let mut basin_sizes = Vec::with_capacity(10);
    for point in low_points(data).keys() {
        let mut basin: HashSet<(usize, usize)> = HashSet::with_capacity(10);
        grow_basin(data, point, &mut basin);
        basin_sizes.push(basin.len());
    }
    basin_sizes.sort_unstable();

    basin_sizes.pop().unwrap() as u32
        * basin_sizes.pop().unwrap() as u32
        * basin_sizes.pop().unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: [[u8; 10]; 5] = [
        [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
        [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
        [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
        [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
        [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
    ];

    #[test]
    fn test_star1() {
        let sample_data: Vec<&[u8]> = SAMPLE_DATA.iter().map(|r| r.as_slice()).collect();
        assert_eq!(star1(&sample_data), 15);
    }

    #[test]
    fn test_star2() {
        let sample_data: Vec<&[u8]> = SAMPLE_DATA.iter().map(|r| r.as_slice()).collect();
        assert_eq!(star2(&sample_data), 1134);
    }
}
