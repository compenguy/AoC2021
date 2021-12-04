use anyhow::Result;
use std::io::BufRead;

const DATA_FILE: &str = "3.txt";

pub fn data<P: AsRef<std::path::Path>>(data_dir: P) -> Result<Vec<u32>> {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let data = std::io::BufReader::new(std::fs::File::open(&data_file)?);
    data.lines()
        .map(|s_res| {
            s_res
                .map_err(|e| e.into())
                .and_then(|s| u32::from_str_radix(s.as_str(), 2).map_err(|e| e.into()))
        })
        .collect()
}

fn bit_set(word: u32, index: u8) -> bool {
    (word & (1 << (31 - index))) != 0
}

pub fn power_consumption(data: &[u32], bit_width: u8) -> (u32, u32) {
    let offset: usize = (32 - bit_width).into();
    // count how many 1s we see in a particular bit position,
    // for all bit positions
    let counts = data
        .iter()
        .fold(vec![0; bit_width.into()], |mut counts, word| {
            for i in offset..32 {
                counts[i - offset] += bit_set(*word, i as u8) as u32;
            }
            counts
        });

    // split_point is half the total number of elements, for deciding
    // whether the number of 1s we counted represents the majority
    // elements, than it's the greater
    let split_point: u32 = data.len() as u32 / 2;

    let mut epsilon = 0;
    for count in &counts {
        epsilon <<= 1;
        epsilon |= (*count > split_point) as u32;
    }
    let mut gamma = !epsilon;
    gamma &= (1 << counts.len()) - 1;
    (epsilon, gamma)
}

pub fn star1(data: &[u32]) -> u64 {
    let (gamma, epsilon) = power_consumption(data, 12);
    (gamma as u64) * (epsilon as u64)
}

#[allow(dead_code)]
fn search_linear(data: &[u32], bit_pattern: u32, bit_offset: u8, greater: bool) -> u32 {
    if bit_offset == 32 || data.len() == 1 {
        return bit_pattern;
    }
    // Need to mask for only the bits we've already considered
    let offset_mask = std::u32::MAX << (32 - bit_offset);
    let (total, count) = data
        .iter()
        .filter(|&&w| (w & offset_mask) == (bit_pattern & offset_mask))
        .fold((0, 0), |(total, count), word| {
            (total + 1, count + (bit_set(*word, bit_offset) as u32))
        });

    let winner = if count == 0 {
        0
    } else if count == total || ((2 * count) >= total) == greater {
        1
    } else {
        0
    };

    let new_bit_pattern = bit_pattern | ((winner as u32) << (31 - bit_offset));

    search_linear(data, new_bit_pattern, bit_offset + 1, greater)
}

#[allow(dead_code)]
fn search(data: &mut [u32], bit_offset: u8, greater: bool) -> u32 {
    // Recursion terminating condition - we've found the entry with
    // either the most common bit for its field, or the least common
    if data.len() == 1 {
        return data[0];
    }

    // split_point is the last value in the sorted list with a 0
    // in the requisite position
    let split_point = data.partition_point(|&x| !bit_set(x, bit_offset));
    let (zeros, ones) = data.split_at_mut(split_point);

    // Figure out which group is bigger - bit-set vs bit-unset
    let winner = match (ones.len() >= zeros.len(), greater) {
        (true, true) => ones,
        (false, false) => ones,
        (true, false) => zeros,
        (false, true) => zeros,
    };

    // Sort the subset of the array by whether the bit field of interest
    // is set or not
    winner.sort_unstable_by_key(|x| bit_set(*x, bit_offset + 1));
    search(winner, bit_offset + 1, greater)
}

pub fn star2(data: &mut [u32]) -> u64 {
    let bit_offset = 32 - 12;

    // Sort the array by whether the first bit of interest
    // is set or not
    data.sort_unstable_by_key(|x| bit_set(*x, bit_offset));
    let o2 = search_linear(data, 0, bit_offset, true);
    let co2 = search_linear(data, 0, bit_offset, false);
    (o2 as u64) * (co2 as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: [u32; 12] = [
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ];

    #[test]
    fn test_gamma_epsilon() {
        let (gamma, epsilon) = power_consumption(&SAMPLE_DATA, 5);
        assert_eq!(gamma, 0b00010110);
        assert_eq!(epsilon, 0b00001001);
        assert_eq!((gamma as u64) * (epsilon as u64), 198u64);
    }

    #[test]
    fn test_o2_co2() {
        let mut sample_data = SAMPLE_DATA.clone();
        sample_data.sort_unstable_by_key(|x| bit_set(*x, 32 - 5));
        let o2 = search(&mut sample_data, 32 - 5, true);
        let co2 = search(&mut sample_data, 32 - 5, false);
        assert_eq!(o2, 0b00010111);
        assert_eq!(co2, 0b00001010);
        assert_eq!((o2 as u64) * (co2 as u64), 230u64);
    }

    #[test]
    fn test_o2_co2_linear() {
        let mut sample_data = SAMPLE_DATA.clone();
        println!("Searching for O2...");
        let o2 = search_linear(&mut sample_data, 0, 32 - 5, true);
        assert_eq!(o2, 0b00010111);
        println!("Searching for CO2...");
        let co2 = search_linear(&mut sample_data, 0, 32 - 5, false);
        assert_eq!(co2, 0b00001010);
        println!("Testing life support...");
        assert_eq!((o2 as u64) * (co2 as u64), 230u64);
    }
}
