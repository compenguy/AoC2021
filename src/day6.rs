use anyhow::Result;
use std::io::BufRead;

const DATA_FILE: &str = "6.txt";

#[derive(Debug, Clone)]
struct FishPond {
    pub fish: Vec<u64>,
}

impl std::convert::From<&[u8]> for FishPond {
    fn from(data: &[u8]) -> Self {
        let mut pond = FishPond { fish: vec![0; 10] };

        for val in data {
            pond.fish[*val as usize] += 1;
        }
        pond
    }
}

impl FishPond {
    fn tick(&mut self) {
        let splitting = self.fish[0];
        self.fish[0] = self.fish[1];
        self.fish[1] = self.fish[2];
        self.fish[2] = self.fish[3];
        self.fish[3] = self.fish[4];
        self.fish[4] = self.fish[5];
        self.fish[5] = self.fish[6];
        self.fish[6] = self.fish[7];
        self.fish[7] = self.fish[8];
        self.fish[6] += splitting;
        self.fish[8] = splitting;
    }

    fn count(&self) -> u64 {
        self.fish.iter().cloned().sum::<u64>()
    }

    #[allow(dead_code)]
    fn remaining(&self) -> Vec<u8> {
        let mut remaining: Vec<u8> = Vec::with_capacity(self.count() as usize);
        for (time, count) in self.fish.iter().enumerate() {
            for _ in 0..*count {
                remaining.push(time as u8);
            }
        }
        remaining
    }
}

pub fn data<P: AsRef<std::path::Path>>(data_dir: P) -> Result<Vec<u8>> {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let data = std::io::BufReader::new(std::fs::File::open(&data_file)?);
    data.split(b',')
        .map(|b_res| {
            b_res
                .map_err(|e| e.into())
                .and_then(|b| String::from_utf8(b).map_err(|e| e.into()))
                .and_then(|s| s.trim().parse::<u8>().map_err(|e| e.into()))
        })
        .collect()
}

pub fn star1(data: &[u8]) -> u64 {
    let mut pond = FishPond::from(data);
    for _ in 1..=80 {
        pond.tick();
    }
    pond.count()
}

pub fn star2(data: &[u8]) -> u64 {
    let mut pond = FishPond::from(data);
    for _ in 1..=256 {
        pond.tick();
    }
    pond.count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: [u8; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn test_star1() {
        let mut pond = FishPond::from(SAMPLE_DATA.as_slice());
        assert_eq!(pond.remaining(), vec![1, 2, 3, 3, 4], "Initial");
        pond.tick();

        assert_eq!(pond.remaining(), vec![0, 1, 2, 2, 3], "After 1 day");
        pond.tick();
        assert_eq!(pond.remaining(), vec![0, 1, 1, 2, 6, 8], "After 2 days");
        pond.tick();
        assert_eq!(pond.remaining(), vec![0, 0, 1, 5, 6, 7, 8], "After 3 days");
        pond.tick();
        assert_eq!(
            pond.remaining(),
            vec![0, 4, 5, 6, 6, 6, 7, 8, 8],
            "After 4 days"
        );

        for _ in 5..=18 {
            pond.tick();
        }
        println!("After 18 days: {:?}", pond.remaining());
        assert_eq!(pond.count(), 26, "after 18 days");
        for _ in 19..=80 {
            pond.tick();
        }
        assert_eq!(pond.count(), 5934, "after 40 days");
    }

    #[test]
    fn test_star2() {
        let mut pond = FishPond::from(SAMPLE_DATA.as_slice());
        for _ in 1..=256 {
            pond.tick();
        }
        assert_eq!(pond.count(), 26984457539);
    }
}
