use anyhow::Result;
use std::io::BufRead;

fn count_larger(vals: impl Iterator<Item = u32>) -> u32 {
    let (count, _) = vals.fold((0, std::u32::MAX), |(count, last), cur| {
        if cur > last {
            (count + 1, cur)
        } else {
            (count, cur)
        }
    });
    count
}

pub(crate) fn day1<P: AsRef<std::path::Path>>(data_dir: P) -> Result<u32> {
    let data_file = data_dir.as_ref().join("1.txt");
    let data = std::io::BufReader::new(std::fs::File::open(&data_file)?);
    let numeric_data = data.lines().map(|s_res| {
        s_res
            .unwrap_or_else(|_| panic!("file I/O error in {}", data_file.display()))
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("data format error in {}", data_file.display()))
    });
    Ok(count_larger(numeric_data))
}
