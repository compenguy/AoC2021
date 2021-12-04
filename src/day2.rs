use anyhow::{anyhow, Error, Result};
use std::convert::TryFrom;
use std::io::BufRead;

const DATA_FILE: &str = "2.txt";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Travel {
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

pub fn data<P: AsRef<std::path::Path>>(data_dir: P) -> Result<Vec<Travel>> {
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

pub fn star1(data: &[Travel]) -> u32 {
    let (x, y) = data
        .iter()
        .fold((0i32, 0i32), |(x, y), action| match action {
            Travel::Forward(dx) => (x + dx, y),
            Travel::Depth(dy) => (x, y + dy),
        });
    (x * y).wrapping_abs() as u32
}

pub fn star2(data: &[Travel]) -> u32 {
    let (x, y, _) = data
        .iter()
        .fold((0i32, 0i32, 0i32), |(x, y, aim), action| match action {
            Travel::Forward(dx) => (x + dx, y + (aim * dx), aim),
            Travel::Depth(dy) => (x, y, aim + dy),
        });
    (x * y).wrapping_abs() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    const SAMPLE_DATA: [&'static str; 6] = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    const SAMPLE_DATA_CONVERTED: [Travel; 6] = [
        Travel::Forward(5),
        Travel::Depth(5),
        Travel::Forward(8),
        Travel::Depth(-3),
        Travel::Depth(8),
        Travel::Forward(2),
    ];

    #[test]
    fn test_travel_conversion() {
        for (inp, out) in SAMPLE_DATA.iter().zip(SAMPLE_DATA_CONVERTED.iter()) {
            assert_eq!(Travel::try_from(*inp).unwrap(), *out);
        }
    }

    #[test]
    fn test_star1() {
        assert_eq!(star1(&SAMPLE_DATA_CONVERTED), 150);
    }

    #[test]
    fn test_star2() {
        assert_eq!(star2(&SAMPLE_DATA_CONVERTED), 900);
    }
}
