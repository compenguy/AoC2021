use anyhow::Result;
use std::collections::HashMap;
use std::io::BufRead;

const DATA_FILE: &str = "5.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: u32,
    y: u32,
}

impl std::convert::From<(u32, u32)> for Point {
    fn from(text: (u32, u32)) -> Self {
        Self {
            x: text.0,
            y: text.1,
        }
    }
}

impl std::str::FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        s.split_once(',')
            .ok_or_else(|| anyhow::anyhow!("Error splitting point coordinates in input data"))
            .and_then(|(s1, s2)| {
                s1.parse::<u32>()
                    .and_then(|s1| s2.parse::<u32>().map(|s2| (s1, s2)))
                    .map_err(|e| e.into())
            })
            .map(Point::from)
    }
}

#[derive(Debug, Clone)]
pub struct Range {
    pub start: Point,
    pub end: Point,
}

impl std::convert::From<(Point, Point)> for Range {
    fn from(points: (Point, Point)) -> Self {
        Self {
            start: points.0,
            end: points.1,
        }
    }
}

impl std::str::FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        s.split_once(" -> ")
            .ok_or_else(|| anyhow::anyhow!("Error splitting endpoints in input data"))
            .and_then(|(s1, s2)| {
                s1.parse::<Point>()
                    .and_then(|s1| s2.parse::<Point>().map(|s2| (s1, s2)))
            })
            .map(Range::from)
    }
}

impl Range {
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = Point> + 'a> {
        let x_range: Vec<u32> = if self.end.x > self.start.x {
            (self.start.x..=self.end.x).collect()
        } else {
            (self.end.x..=self.start.x).rev().collect()
        };
        let y_range: Vec<u32> = if self.end.y > self.start.y {
            (self.start.y..=self.end.y).collect()
        } else {
            (self.end.y..=self.start.y).rev().collect()
        };
        if x_range.len() == y_range.len() {
            Box::new(
                x_range
                    .into_iter()
                    .zip(y_range.into_iter())
                    .map(Point::from),
            )
        } else if x_range.len() == 1 {
            Box::new(
                std::iter::repeat(x_range[0])
                    .zip(y_range.into_iter())
                    .map(Point::from),
            )
        } else if y_range.len() == 1 {
            Box::new(
                x_range
                    .into_iter()
                    .zip(std::iter::repeat(y_range[0]))
                    .map(Point::from),
            )
        } else {
            unreachable!();
        }
    }

    fn horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}

pub fn data<P: AsRef<std::path::Path>>(data_dir: P) -> Result<Vec<Range>> {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let data = std::io::BufReader::new(std::fs::File::open(&data_file)?);
    data.lines()
        .map(|s_res| s_res.map_err(|e| e.into()).and_then(|s| s.parse::<Range>()))
        .collect()
}

pub fn star1(data: &[Range]) -> u32 {
    let mut occupied_points: HashMap<Point, u32> = HashMap::with_capacity(256);
    for point in data
        .iter()
        .filter(|r| r.horizontal() || r.vertical())
        .flat_map(|r| r.iter())
    {
        let point_count = occupied_points.entry(point).or_insert(0);
        *point_count += 1;
    }
    occupied_points.values().filter(|&x| *x >= 2).count() as u32
}

pub fn star2(data: &[Range]) -> u32 {
    let mut occupied_points: HashMap<Point, u32> = HashMap::with_capacity(256);
    for point in data.iter().flat_map(|r| r.iter()) {
        let point_count = occupied_points.entry(point).or_insert(0);
        *point_count += 1;
    }
    occupied_points.values().filter(|&x| *x >= 2).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: [(Point, Point); 10] = [
        (Point { x: 0, y: 9 }, Point { x: 5, y: 9 }),
        (Point { x: 8, y: 0 }, Point { x: 0, y: 8 }),
        (Point { x: 9, y: 4 }, Point { x: 3, y: 4 }),
        (Point { x: 2, y: 2 }, Point { x: 2, y: 1 }),
        (Point { x: 7, y: 0 }, Point { x: 7, y: 4 }),
        (Point { x: 6, y: 4 }, Point { x: 2, y: 0 }),
        (Point { x: 0, y: 9 }, Point { x: 2, y: 9 }),
        (Point { x: 3, y: 4 }, Point { x: 1, y: 4 }),
        (Point { x: 0, y: 0 }, Point { x: 8, y: 8 }),
        (Point { x: 5, y: 5 }, Point { x: 8, y: 2 }),
    ];

    #[test]
    fn test_star1() {
        let sample_data: Vec<Range> = SAMPLE_DATA.iter().cloned().map(Range::from).collect();
        assert_eq!(star1(&sample_data), 5);
    }

    /*
    #[test]
    fn test_star2() {
        assert_eq!(star2(&SAMPLE_DATA), 5);
    }
    */
}
