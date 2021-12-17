use std::collections::HashSet;
use std::io::BufRead;

const DATA_FILE: &str = "13.txt";

#[derive(Debug, Clone)]
pub enum AxisFold {
    X(u32),
    Y(u32),
}

impl AxisFold {
    fn apply(&self, points: &HashSet<(u32, u32)>) -> HashSet<(u32, u32)> {
        points.iter().map(|p| self.axis_fold(p)).collect()
    }

    fn axis_fold(&self, point: &(u32, u32)) -> (u32, u32) {
        let (x, y) = point;
        match self {
            AxisFold::X(n) if x > n => ((2 * *n) - *x, *y),
            AxisFold::Y(n) if y > n => (*x, (2 * *n) - *y),
            _ => (*x, *y),
        }
    }
}

pub fn parse<I: Iterator<Item = String>>(mut data: I) -> (HashSet<(u32, u32)>, Vec<AxisFold>) {
    let mut points: HashSet<(u32, u32)> = HashSet::with_capacity(50);
    let mut folds: Vec<AxisFold> = Vec::with_capacity(10);

    for line in data.by_ref() {
        if line.trim().is_empty() {
            break;
        }
        points.insert(
            line.split_once(',')
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap(),
        );
    }

    for line in data {
        assert!(line.starts_with("fold along "));
        let fold_str = line.strip_prefix("fold along ").map(|s| s.trim()).unwrap();
        let fold = fold_str
            .split_once('=')
            .map(|a| match a.0 {
                "x" => AxisFold::X(a.1.parse().unwrap()),
                "y" => AxisFold::Y(a.1.parse().unwrap()),
                _ => panic!(),
            })
            .unwrap();
        folds.push(fold);
    }

    (points, folds)
}

pub fn data<P: AsRef<std::path::Path>>(data_dir: P) -> impl Iterator<Item = String> {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let data = std::io::BufReader::new(std::fs::File::open(&data_file).unwrap());
    data.lines().map(|s_res| s_res.unwrap())
}

pub fn star1(data: (HashSet<(u32, u32)>, &[AxisFold])) -> u32 {
    let (points, folds) = data;
    let final_field = folds
        .iter()
        .take(1)
        .fold(points, |points, fold| fold.apply(&points));
    final_field.len() as u32
}

pub fn star2(data: (HashSet<(u32, u32)>, &[AxisFold])) -> u32 {
    let (points, folds) = data;
    let final_field = folds
        .iter()
        .fold(points, |points, fold| fold.apply(&points));

    let max_x = final_field.iter().max_by_key(|(x, _)| x).unwrap().0;
    let max_y = final_field.iter().max_by_key(|(_, y)| y).unwrap().1;
    for y in 0..=max_y {
        for x in 0..=max_x {
            if final_field.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    final_field.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: [&'static str; 21] = [
        "6,10",
        "0,14",
        "9,10",
        "0,3",
        "10,4",
        "4,11",
        "6,0",
        "6,12",
        "4,1",
        "0,13",
        "10,12",
        "3,4",
        "3,0",
        "8,4",
        "1,10",
        "2,14",
        "8,10",
        "9,0",
        "",
        "fold along y=7",
        "fold along x=5",
    ];

    #[test]
    fn test_star1() {
        let (points, folds) = parse(SAMPLE_DATA.iter().map(|r| r.to_string()));
        assert_eq!(star1((points.clone(), &folds)), 17);
    }

    #[test]
    fn test_star2() {
        let (points, folds) = parse(SAMPLE_DATA.iter().map(|r| r.to_string()));
        assert_eq!(star2((points.clone(), &folds)), 16);
    }
}
