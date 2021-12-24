use std::io::BufRead;
use std::ops::RangeInclusive;

const DATA_FILE: &str = "17.txt";

pub fn data<P: AsRef<std::path::Path>>(data_dir: P) -> String {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let data = std::io::BufReader::new(std::fs::File::open(&data_file).unwrap());
    data.lines().map(|s_res| s_res.unwrap()).next().unwrap()
}

pub fn parse(data: &str) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
    let ranges = data.strip_prefix("target area: ").unwrap();
    let (x_range_str, y_range_str) = ranges
        .split_once(", ")
        .map(|(x, y)| (x.to_string(), (y.to_string())))
        .unwrap();
    let (x_lower, x_upper) = x_range_str
        .strip_prefix("x=")
        .unwrap()
        .split_once("..")
        .map(|(l, u)| (l.parse::<isize>().unwrap(), u.parse::<isize>().unwrap()))
        .unwrap();
    let (y_lower, y_upper) = y_range_str
        .strip_prefix("y=")
        .unwrap()
        .split_once("..")
        .map(|(l, u)| (l.parse::<isize>().unwrap(), u.parse::<isize>().unwrap()))
        .unwrap();

    // Handle negative ranges properly - rust ranges don't radiate from 0,
    // they're always from strictly lower to strictly greater
    let x_range = x_lower.min(x_upper)..=x_lower.max(x_upper);
    let y_range = y_lower.min(y_upper)..=y_lower.max(y_upper);
    (x_range, y_range)
}

fn target_distance(a: isize, a_range: &RangeInclusive<isize>) -> isize {
    if a_range.contains(&a) {
        //println!("[within] {:3} <= {:3} <= {:3}", a_range.start(), a, a_range.end());
        0
    } else if a < *a_range.start() {
        //println!("[short ] a: {:3} a_lower: {:3} -> {:3}", a, a_range.start(), a - a_range.start());
        a - a_range.start()
    } else if a > *a_range.end() {
        //println!("[past  ] a: {:3} a_upper: {:3} -> {:3}", a, a_range.end(), a - a_range.end());
        a - a_range.end()
    } else {
        unreachable!();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
    InFlight(isize, isize),
    Miss(isize, isize),
    Kaboom,
}

impl Outcome {
    fn in_flight(&self) -> bool {
        matches!(self, Outcome::InFlight(_, _))
    }
}

#[derive(Debug, Clone)]
struct War {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
    max_y: isize,
    finished: bool,
}

impl War {
    fn start(
        dx: isize,
        dy: isize,
        x_range: RangeInclusive<isize>,
        y_range: RangeInclusive<isize>,
    ) -> Self {
        Self {
            x: 0,
            y: 0,
            dx,
            dy,
            x_range,
            y_range,
            max_y: 0,
            finished: false,
        }
    }

    fn repeat(&mut self, dx: isize, dy: isize) -> &mut Self {
        self.x = 0;
        self.y = 0;
        self.dx = dx;
        self.dy = dy;
        self.max_y = self.y;
        self.finished = false;
        self
    }

    fn sighting(&self) -> Outcome {
        let x_dist = target_distance(self.x, &self.x_range);
        let y_dist = target_distance(self.y, &self.y_range);

        if x_dist == 0 && y_dist == 0 {
            Outcome::Kaboom
        } else if x_dist > 0 {
            // dx can never be negative, so if the shot position is higher than the target x upper limit,
            // we've overshot
            if cfg!(debug_assertions) {
                println!("X overshoot");
            }
            Outcome::Miss(x_dist, y_dist)
        } else if x_dist < 0 && self.dx == 0 {
            // X will make no more progress, and it's short of the target
            if cfg!(debug_assertions) {
                println!("X undershoot");
            }
            Outcome::Miss(x_dist, y_dist)
        } else if y_dist < 0 && self.dy < 0 {
            // dy will always eventually flip from positive to negative, so if y is lower than the lower
            // limit and dy is negative, we've overshot
            //println!("Y overshoot");
            Outcome::Miss(x_dist, y_dist)
        } else {
            Outcome::InFlight(x_dist, y_dist)
        }
    }

    fn step(&mut self) -> Outcome {
        self.x += self.dx;
        self.y += self.dy;
        self.max_y = self.max_y.max(self.y);

        let outcome = self.sighting();
        self.finished = !outcome.in_flight();

        self.dx = if self.dx - 1 < 0 { 0 } else { self.dx - 1 };
        self.dy -= 1;

        outcome
    }

    fn max_y(&self) -> isize {
        self.max_y
    }
}

fn its_war_then(
    x_range: &RangeInclusive<isize>,
    y_range: &RangeInclusive<isize>,
) -> (isize, usize) {
    let mut max_y = std::isize::MIN;
    let mut firing_solutions: usize = 0;

    let mut war = War::start(0, 0, x_range.clone(), y_range.clone());
    for dx in 0..=1000isize {
        for dy in -1000..=1000isize {
            //println!("Starting war: dx: {:3} dy: {:3}", dx, dy);
            war.repeat(dx, dy);
            let mut outcome = war.step();
            while outcome.in_flight() {
                //println!("\t{:?} => {:?}", &war, &outcome);
                outcome = war.step();
            }
            //println!("\t{:?} => {:?}", &war, &outcome);
            if let Outcome::Miss(_dist_x, _dist_y) = outcome {
                //println!("Missed it by [{}, {}] that much", dist_x, dist_y);
            } else {
                firing_solutions += 1;
                max_y = max_y.max(war.max_y());
            }
        }
    }
    (max_y, firing_solutions)
}

pub fn star1(ranges: &(RangeInclusive<isize>, RangeInclusive<isize>)) -> isize {
    its_war_then(&ranges.0, &ranges.1).0
}

pub fn star2(ranges: &(RangeInclusive<isize>, RangeInclusive<isize>)) -> usize {
    its_war_then(&ranges.0, &ranges.1).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_distance() {
        assert_eq!(target_distance(5, &(10isize..=20)), -5);
        assert_eq!(target_distance(5, &(10isize..=100)), -5);
        assert_eq!(target_distance(10, &(10isize..=100)), -0);
        assert_eq!(target_distance(50, &(10isize..=100)), -0);
        assert_eq!(target_distance(100, &(10isize..=100)), -0);
        assert_eq!(target_distance(101, &(10isize..=100)), 1);
        assert_eq!(target_distance(110, &(10isize..=100)), 10);
        assert_eq!(target_distance(110, &(10isize..=20)), 90);

        assert_eq!(target_distance(50, &(-100isize..=-10)), 60);
        assert_eq!(target_distance(10, &(-100isize..=-10)), 20);
        assert_eq!(target_distance(5, &(-100isize..=-10)), 15);
        assert_eq!(target_distance(5, &(-20isize..=-10)), 15);
        assert_eq!(target_distance(-5, &(-20isize..=-10)), 5);
        assert_eq!(target_distance(-10, &(-20isize..=-10)), 0);
        assert_eq!(target_distance(-20, &(-20isize..=-10)), 0);
        assert_eq!(target_distance(-50, &(-100isize..=-10)), 0);
        assert_eq!(target_distance(-100, &(-100isize..=-10)), 0);
        assert_eq!(target_distance(-101, &(-100isize..=-10)), -1);
        assert_eq!(target_distance(-110, &(-100isize..=-10)), -10);
        assert_eq!(target_distance(-110, &(-20isize..=-10)), -90);
    }

    #[test]
    fn test_sighting() {
        let x_range = 10..=20isize;
        let y_range = -20..=-10isize;
        let mut war = War::start(0, 0, x_range, y_range);

        war.repeat(0, 0);
        assert_eq!(war.step(), Outcome::Miss(-10, 10), "war: {:?}", war);
        assert_eq!(war.step(), Outcome::Miss(-10, 9), "war: {:?}", war);
        war.repeat(1, 1);
        assert_eq!(war.step(), Outcome::InFlight(-9, 11), "war: {:?}", war);
        assert_eq!(war.step(), Outcome::Miss(-9, 11), "war: {:?}", war);
        assert_eq!(war.step(), Outcome::Miss(-9, 10), "war: {:?}", war);
        assert_eq!(war.step(), Outcome::Miss(-9, 8), "war: {:?}", war);
        war.repeat(4, -2);
        assert_eq!(war.step(), Outcome::InFlight(-6, 8), "war: {:?}", war);
        assert_eq!(war.step(), Outcome::InFlight(-3, 5), "war: {:?}", war);
        assert_eq!(war.step(), Outcome::InFlight(-1, 1), "war: {:?}", war);
        assert_eq!(war.step(), Outcome::Kaboom, "war: {:?}", war);
    }

    const SAMPLE_DATA: [(&'static str, isize, usize); 1] =
        [("target area: x=20..30, y=-10..-5", 45, 112)];

    #[test]
    fn test_star1() {
        for (_i, (input, output, _)) in SAMPLE_DATA.iter().enumerate() {
            let data = parse(input);
            assert_eq!(star1(&data), *output);
        }
    }

    #[test]
    fn test_star2() {
        for (_i, (input, _, output)) in SAMPLE_DATA.iter().enumerate() {
            let data = parse(input);
            assert_eq!(star2(&data), *output);
        }
    }
}
