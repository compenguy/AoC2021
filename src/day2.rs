use anyhow::{anyhow, Error, Result};
use std::convert::TryFrom;
use std::io::BufRead;

const DATA_FILE: &str = "2.txt";

pub(crate) enum Travel {
    Forward(i32),
    Depth(i32),
}

impl std::convert::TryFrom<&str> for Travel {
    type Error = Error;
    fn try_from(line: &str) -> std::result::Result<Self, Self::Error> {
        match line
            .split_once(' ')
            .ok_or_else(|| anyhow!("Unparseable input line: {}", line))
            .and_then(|(dir, dist)| {
                dist.parse::<i32>()
                    .map(|dist| (dir, dist))
                    .map_err(|e| e.into())
            })? {
            ("forward", dx) => Ok(Self::Forward(dx)),
            ("down", dy) => Ok(Self::Depth(dy)),
            ("up", dy) => Ok(Self::Depth(-dy)),
            (dir, _) => Err(anyhow!("Unrecognized direction: {}", dir)),
        }
    }
}

pub(crate) fn data<P: AsRef<std::path::Path>>(data_dir: P) -> Result<Vec<Travel>> {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let data = std::io::BufReader::new(std::fs::File::open(&data_file)?);
    data.lines()
        .map(|s_res| {
            s_res
                .map_err(|e| e.into())
                .and_then(|s| Travel::try_from(s.as_str()))
        })
        .collect()
}

pub(crate) fn star1(data: &[Travel]) -> u32 {
    let (x, y) = data
        .iter()
        .fold((0i32, 0i32), |(x, y), action| match action {
            Travel::Forward(dx) => (x + dx, y),
            Travel::Depth(dy) => (x, y + dy),
        });
    (x * y).wrapping_abs() as u32
}

pub(crate) fn star2(data: &[Travel]) -> u32 {
    let (x, y, _) = data
        .iter()
        .fold((0i32, 0i32, 0i32), |(x, y, aim), action| match action {
            Travel::Forward(dx) => (x + dx, y + (aim * dx), aim),
            Travel::Depth(dy) => (x, y, aim + dy),
        });
    (x * y).wrapping_abs() as u32
}
