use std::collections::HashMap;
use std::io::BufRead;

const DATA_FILE: &str = "14.txt";

pub fn data<P: AsRef<std::path::Path>>(data_dir: P) -> impl Iterator<Item = String> {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let data = std::io::BufReader::new(std::fs::File::open(&data_file).unwrap());
    data.lines().map(|s_res| s_res.unwrap())
}

pub fn parse<I: Iterator<Item = String>>(mut data: I) -> (Vec<u8>, HashMap<(u8, u8), u8>) {
    let polymer: Vec<u8> = data.next().unwrap().into_bytes();
    let _ = data.next().unwrap();
    let insertions: HashMap<(u8, u8), u8> = data
        .filter_map(|rule| {
            rule.split_once(" -> ")
                .map(|(a, b)| ((a.as_bytes()[0], a.as_bytes()[1]), b.as_bytes()[0]))
        })
        .collect();

    (polymer, insertions)
}

fn apply(polymer: &[u8], insertions: &HashMap<(u8, u8), u8>, count: u32) -> u64 {
    let mut poly_pairs: HashMap<(u8, u8), u64> = HashMap::with_capacity(50);
    for pair in polymer.windows(2) {
        let entry = poly_pairs.entry((pair[0], pair[1])).or_insert(0);
        *entry += 1;
    }
    let mut elements: HashMap<u8, u64> = HashMap::with_capacity(20);
    for element in polymer {
        let entry = elements.entry(*element).or_insert(0);
        *entry += 1;
    }

    for _step in 1..=count {
        let mut new_poly: HashMap<(u8, u8), u64> = HashMap::with_capacity(50);
        for (pair, count) in &poly_pairs {
            let (a, b) = *pair;
            if let Some(ins) = insertions.get(&(a, b)) {
                let left_entry = new_poly.entry((a, *ins)).or_insert(0);
                *left_entry += count;
                let right_entry = new_poly.entry((*ins, b)).or_insert(0);
                *right_entry += count;
                let count_entry = elements.entry(*ins).or_insert(0);
                *count_entry += count;
            }
        }
        poly_pairs.clear();
        poly_pairs.extend(new_poly);
    }
    let max = elements.values().max().unwrap_or(&0);
    //println!("max: {}", max);
    let min = elements.values().min().unwrap_or(&0);
    //println!("min: {}", min);
    max - min
}

pub fn star1(polymer: &[u8], insertions: &HashMap<(u8, u8), u8>) -> u64 {
    apply(polymer, insertions, 10)
}

pub fn star2(polymer: &[u8], insertions: &HashMap<(u8, u8), u8>) -> u64 {
    apply(polymer, insertions, 40)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: [&'static str; 18] = [
        "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B", "HN -> C",
        "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B", "CC -> N",
        "CN -> C",
    ];

    #[test]
    fn test_star1() {
        let (polymer, insertions) = parse(SAMPLE_DATA.iter().map(|r| r.to_string()));
        assert_eq!(star1(&polymer, &insertions), 1588);
    }

    #[test]
    fn test_star2() {
        let (polymer, insertions) = parse(SAMPLE_DATA.iter().map(|r| r.to_string()));
        assert_eq!(star2(&polymer, &insertions), 2188189693529);
    }
}
