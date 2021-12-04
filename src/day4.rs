use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

const DATA_FILE: &str = "4.txt";

pub(crate) fn data<P: AsRef<std::path::Path>>(data_dir: P) -> Result<(Vec<u8>, Vec<Board>)> {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let mut data = std::io::BufReader::new(std::fs::File::open(&data_file)?);
    let mut line = String::with_capacity(50);

    // Read called numbers
    data.read_line(&mut line)?;
    let called: Result<Vec<u8>> = line
        .trim()
        .split(',')
        .map(|s| s.parse::<u8>().map_err(|e| e.into()))
        .collect();
    let called = called?;
    // There's an empty line before starting the board data. Let's consume that

    let mut boards = Vec::with_capacity(25);
    // There are blank lines separating boards
    // We also get empty reads once we reach the end of the file
    // So we're done once we get two board parses in a row that return None
    let mut none_count = 0;
    while none_count <= 1 {
        match parse_board(&mut data)? {
            Some(board) => {
                boards.push(board);
                none_count = 0;
            }
            None => none_count += 1,
        }
    }

    Ok((called, boards))
}

fn parse_board(data: &mut std::io::BufReader<std::fs::File>) -> Result<Option<Board>> {
    let cells: Vec<String> = data
        .lines()
        .filter_map(Result::ok)
        .take_while(|s| !s.is_empty())
        .collect();
    let cells: Result<Vec<u8>> = cells
        .iter()
        .flat_map(|s| s.split(' '))
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().parse::<u8>().map_err(|e| e.into()))
        .collect();
    let cells = cells?;
    if cells.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Board::from(cells.as_slice())))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Board {
    cells: HashMap<u8, (u8, u8, bool)>,
    has_won: bool,
}

impl std::convert::From<&[u8]> for Board {
    fn from(values: &[u8]) -> Self {
        let mut cells: HashMap<u8, (u8, u8, bool)> = HashMap::with_capacity(25);
        let mut values = values.iter();
        for y in 0..5 {
            for x in 0..5 {
                cells.insert(*values.next().expect("Programming error"), (y, x, false));
            }
        }
        Self {
            cells,
            has_won: false,
        }
    }
}

impl Board {
    pub(crate) fn call(&mut self, number: u8) -> Option<u32> {
        if self.has_won {
            return None;
        }
        if let Some((_, _, ref mut filled)) = self.cells.get_mut(&number) {
            *filled = true;
        }
        if let Some((y, x, _)) = self.cells.get(&number).cloned() {
            self.test_won(number, y, x)
        } else {
            None
        }
    }

    fn test_won(&mut self, number: u8, y: u8, x: u8) -> Option<u32> {
        let filled: HashSet<(u8, u8)> = self
            .cells
            .values()
            .filter_map(|(y, x, filled)| if *filled { Some((*y, *x)) } else { None })
            .collect();
        let mut row_won: bool = true;
        let mut col_won: bool = true;
        for line in 0..5 {
            if !filled.contains(&(y, line)) {
                row_won = false;
            }
            if !filled.contains(&(line, x)) {
                col_won = false;
            }
        }

        if row_won || col_won {
            self.has_won = true;
            Some(self.score_board(number))
        } else {
            None
        }
    }

    pub(crate) fn has_won(&self) -> bool {
        self.has_won
    }

    fn score_board(&self, number: u8) -> u32 {
        let unfilled_sum: u32 = self
            .cells
            .iter()
            .filter_map(|(n, (_, _, filled))| if *filled { None } else { Some(*n as u32) })
            .sum();
        (unfilled_sum) * (number as u32)
    }
}

pub(crate) fn star1(called: &[u8], boards: &mut [Board]) -> u32 {
    let mut max_score = 0;
    for number in called {
        max_score = boards
            .iter_mut()
            .filter_map(|b| b.call(*number))
            .fold(max_score, |m, s| m.max(s));
        if max_score > 0 {
            break;
        }
    }
    max_score
}

pub(crate) fn star2(called: &[u8], boards: &mut [Board]) -> u32 {
    let mut min_score: Option<u32> = None;
    for number in called {
        if boards.iter().all(|b| b.has_won()) {
            break;
        }
        min_score = boards
            .iter_mut()
            .filter_map(|b| b.call(*number))
            .fold(None, |m, s| m.map(|m| m.min(s)).or(Some(s)));
    }
    min_score.expect("Programming error")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_CALLED: [u8; 27] = [
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];
    const SAMPLE_BOARDS: [[u8; 25]; 3] = [
        [
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19,
        ],
        [
            3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12,
            6,
        ],
        [
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ],
    ];

    #[test]
    fn test_star1() {
        let mut boards: Vec<Board> = SAMPLE_BOARDS
            .iter()
            .map(|v| Board::from(v.as_slice()))
            .collect();
        assert_eq!(star1(&SAMPLE_CALLED, &mut boards), 4512);
    }
}
