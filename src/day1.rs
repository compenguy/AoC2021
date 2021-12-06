use anyhow::Result;
use std::io::BufRead;

const DATA_FILE: &str = "1.txt";

pub fn data<P: AsRef<std::path::Path>>(data_dir: P) -> Result<Vec<u32>> {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let data = std::io::BufReader::new(std::fs::File::open(&data_file)?);
    data.lines()
        .map(|s_res| {
            s_res
                .map_err(|e| e.into())
                .and_then(|s| s.parse::<u32>().map_err(|e| e.into()))
        })
        .collect()
}

pub fn star1(data: &[u32]) -> u32 {
    data.windows(2)
        .filter(|window| window[1] > window[0])
        .count() as u32
}

pub fn star2(data: &[u32]) -> u32 {
    let sums: Vec<u32> = data.windows(3).map(|d| d.iter().sum()).collect();
    sums.windows(2)
        .filter(|window| window[1] > window[0])
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_star1() {
        assert_eq!(star1(&SAMPLE_DATA), 7);
    }

    #[test]
    fn test_star2() {
        assert_eq!(star2(&SAMPLE_DATA), 5);
    }
}
