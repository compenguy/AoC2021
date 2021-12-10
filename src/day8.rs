use anyhow::{anyhow, Error, Result};
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::io::BufRead;

const DATA_FILE: &str = "8.txt";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SevSeg {
    pub(crate) wires: u8,
}

impl std::convert::From<&str> for SevSeg {
    fn from(grouping: &str) -> Self {
        let wires: u8 = grouping
            .bytes()
            .map(|b| b - b'a')
            .map(|i| 1 << i)
            .fold(0, |acc, i| acc | i);
        Self { wires }
    }
}

impl SevSeg {
    fn len(&self) -> usize {
        let mut len: u8 = self.wires;
        len = (len & 0x55) + ((len & 0xAA) >> 1);
        len = (len & 0x33) + ((len & 0xCC) >> 2);
        len = (len & 0x0F) + ((len & 0xF0) >> 4);
        len as usize
    }

    fn difference(&self, other: &Self) -> Self {
        Self {
            wires: self.wires & !other.wires,
        }
    }

    fn intersection(&self, other: &Self) -> Self {
        Self {
            wires: self.wires & other.wires,
        }
    }

    fn union(&self, other: &Self) -> Self {
        Self {
            wires: self.wires | other.wires,
        }
    }

    fn is_superset(&self, other: &Self) -> bool {
        self.intersection(other) == *other
    }
}

#[derive(Debug, Clone)]
pub struct SevSegMapping {
    digits: HashSet<SevSeg>,
    display: Vec<SevSeg>,
}

impl std::convert::TryFrom<&str> for SevSegMapping {
    type Error = Error;

    fn try_from(line: &str) -> std::result::Result<Self, Self::Error> {
        let (left, right) = line
            .trim()
            .split_once('|')
            .ok_or_else(|| anyhow!("Unparseable input line: {}", line))?;
        let digits = left.split_whitespace().map(SevSeg::from).collect();
        let display = right.split_whitespace().map(SevSeg::from).collect();
        Ok(Self { digits, display })
    }
}

impl SevSegMapping {
    pub fn solve(&self) -> u32 {
        let mut mapping: HashMap<u8, u8> = HashMap::with_capacity(10);
        let one = self.digits.iter().find(|w| w.len() == 2).unwrap();
        mapping.insert(one.wires, 1);
        let four = self.digits.iter().find(|w| w.len() == 4).unwrap();
        mapping.insert(four.wires, 4);
        let seven = self.digits.iter().find(|w| w.len() == 3).unwrap();
        mapping.insert(seven.wires, 7);
        let eight = self.digits.iter().find(|w| w.len() == 7).unwrap();
        mapping.insert(eight.wires, 8);

        // one is composed of (canonical) segments
        //   2 [c]
        //   5 [f]
        // four is composed of (canonical) segments
        //   1 [b]
        //   2 [c]
        //   3 [d]
        //   5 [f]
        // seven is composed of (canonical) segments
        //   0 [a]
        //   2 [c]
        //   5 [f]
        // eight is composed of (canonical) segments
        //   0 [a]
        //   1 [b]
        //   2 [c]
        //   3 [d]
        //   4 [e]
        //   5 [f]
        //   6 [g]

        // Segment 0 [a] can be definitively identified from digits 1 and 7
        //let a = seven.difference(one);

        // Digit 6 can be definitively identified from digits 1 and 8
        let c_f = one.intersection(eight);

        let six = self
            .digits
            .iter()
            .find(|w| w.len() == 6 && !w.is_superset(one))
            .unwrap();
        mapping.insert(six.wires, 6);

        // Segments 2 [c] and 5 [f] can be definitively identified from digits 6 and 8
        let c = eight.difference(six);
        let f = c_f.difference(&c);

        // Digits 2, 3, and 5 can be definitively identified from Segments 2 and 5
        let two = self
            .digits
            .iter()
            .find(|w| w.len() == 5 && w.is_superset(&c) && !w.is_superset(&f))
            .unwrap();
        mapping.insert(two.wires, 2);
        let three = self
            .digits
            .iter()
            .find(|w| w.len() == 5 && w.is_superset(&c) && w.is_superset(&f))
            .unwrap();
        mapping.insert(three.wires, 3);
        let five = self
            .digits
            .iter()
            .find(|w| w.len() == 5 && !w.is_superset(&c) && w.is_superset(&f))
            .unwrap();
        mapping.insert(five.wires, 5);

        // Digit 9 can be definitively identified from digit 5 and Segment 2 [c]
        let nine = self
            .digits
            .iter()
            .find(|&w| w.len() == 6 && *w == five.union(&c))
            .unwrap();
        mapping.insert(nine.wires, 9);

        // Segment 4 [e] can be definitively identified from digits 6 and 9
        let e = six.difference(nine);

        // Digit 0 can be definitively identified from segments 2 and 4
        let zero = self
            .digits
            .iter()
            .find(|w| w.len() == 6 && w.is_superset(&c) && w.is_superset(&e))
            .unwrap();
        mapping.insert(zero.wires, 0);

        self.display
            .iter()
            .map(|s| s.wires)
            .map(|w| mapping.get(&w).expect("No mapping for digit"))
            .fold(0, |acc, d| (acc * 10) + (*d) as u32)
    }
}

pub fn data<P: AsRef<std::path::Path>>(data_dir: P) -> Result<Vec<SevSegMapping>> {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let data = std::io::BufReader::new(std::fs::File::open(&data_file)?);
    let result = data
        .lines()
        .map(|b_res| {
            b_res
                .map_err(|e| e.into())
                .and_then(|b| SevSegMapping::try_from(b.as_str()))
        })
        .collect::<Result<Vec<SevSegMapping>>>()?;
    Ok(result)
}

fn count_unique(data: &[SevSegMapping]) -> u32 {
    data.iter()
        .flat_map(|m| m.display.iter())
        .filter(|d| matches!(d.len(), 2 | 3 | 4 | 7))
        .count() as u32
}

pub fn star1(data: &[SevSegMapping]) -> u32 {
    count_unique(data)
}

pub fn star2(data: &[SevSegMapping]) -> u32 {
    data.iter().map(|ssm| ssm.solve()).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    const SAMPLE_DATA: [&'static str; 10] = [
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];

    #[test]
    fn test_star1() {
        let sample_data: Result<Vec<SevSegMapping>> = SAMPLE_DATA
            .iter()
            .cloned()
            .map(SevSegMapping::try_from)
            .collect();
        let sample_data = sample_data.unwrap();
        assert_eq!(star1(&sample_data), 26);
    }

    #[test]
    fn test_star2() {
        let sample_data: Result<Vec<SevSegMapping>> = (SAMPLE_DATA)
            .iter()
            .cloned()
            .map(SevSegMapping::try_from)
            .collect();
        let sample_data = sample_data.unwrap();
        assert_eq!(star2(&sample_data), 61229);
    }
}
