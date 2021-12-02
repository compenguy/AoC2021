use anyhow::Result;
use std::io::BufRead;

const DATA_FILE: &str = "1.txt";

pub(crate) fn data<P: AsRef<std::path::Path>>(data_dir: P) -> Result<Vec<u32>> {
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

pub(crate) fn star1(data: &[u32]) -> u32 {
    data.windows(2).fold(0, |count, window| {
        if window[1] > window[0] {
            count + 1
        } else {
            count
        }
    })
}

pub(crate) fn star2(data: &[u32]) -> u32 {
    let (count, _) = data.windows(3).fold(
        (0u32, std::u32::MAX),
        |(count, last_sum), window: &[u32]| {
            let new_sum = window[0] + window[1] + window[2];
            if new_sum > last_sum {
                (count + 1, new_sum)
            } else {
                (count, new_sum)
            }
        },
    );
    count
}
