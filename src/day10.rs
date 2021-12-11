use anyhow::Result;
use std::io::BufRead;

const DATA_FILE: &str = "10.txt";

pub fn data<P: AsRef<std::path::Path>>(data_dir: P) -> Result<Vec<String>> {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let data = std::io::BufReader::new(std::fs::File::open(&data_file)?);
    data.lines()
        .map(|s_res| s_res.map_err(|e| e.into()))
        .collect()
}

fn bracket_match(b: u8) -> u8 {
    match b {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        b'<' => b'>',
        b')' => b'(',
        b']' => b'[',
        b'}' => b'{',
        b'>' => b'<',
        _ => panic!(),
    }
}

fn validate_line(line: &str) -> std::result::Result<String, u8> {
    let mut open_brackets: Vec<u8> = Vec::with_capacity(10);
    for byte in line.bytes() {
        match byte {
            b'(' | b'[' | b'{' | b'<' => open_brackets.push(byte),
            b')' | b']' | b'}' | b'>' => {
                if open_brackets.last() == Some(&bracket_match(byte)) {
                    open_brackets.pop();
                } else {
                    return Err(byte);
                }
            }
            _ => panic!(),
        }
    }
    let close_brackets: String = open_brackets
        .into_iter()
        .rev()
        .map(bracket_match)
        .map(|b| b as char)
        .collect();
    Ok([line, close_brackets.as_str()].concat())
}

pub fn star1(data: &[String]) -> u32 {
    data.iter()
        .filter_map(|l| validate_line(l).err())
        .map(|b| match b {
            b')' => 3u32,
            b']' => 57u32,
            b'}' => 1197u32,
            b'>' => 25137u32,
            _ => panic!(),
        })
        .sum()
}

pub fn star2(data: &[String]) -> u64 {
    let mut scores: Vec<u64> = data
        .iter()
        .filter_map(|l| {
            validate_line(l)
                .ok()
                .and_then(|c| c.strip_prefix(l).map(|s| s.to_string()))
        })
        .map(|c| {
            c.bytes().fold(0u64, |acc, b| {
                (acc * 5)
                    + match b {
                        b')' => 1u64,
                        b']' => 2u64,
                        b'}' => 3u64,
                        b'>' => 4u64,
                        _ => panic!(),
                    }
            })
        })
        .collect();
    scores.sort_unstable();
    scores[(scores.len() / 2)]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: [&'static str; 10] = [
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    #[test]
    fn test_star1() {
        let sample_data: Vec<String> = SAMPLE_DATA.iter().map(|r| r.to_string()).collect();
        assert_eq!(star1(&sample_data), 26397);
    }

    #[test]
    fn test_star2() {
        let sample_data: Vec<String> = SAMPLE_DATA.iter().map(|r| r.to_string()).collect();
        assert_eq!(star2(&sample_data), 288957);
    }
}
