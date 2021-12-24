use std::io::BufRead;
use std::ops::Add;

const DATA_FILE: &str = "18.txt";

pub fn data<P: AsRef<std::path::Path>>(data_dir: P) -> impl Iterator<Item = String> {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let data = std::io::BufReader::new(std::fs::File::open(&data_file).unwrap());
    data.lines().map(|s_res| s_res.unwrap())
}

pub fn parse<I: Iterator<Item = String>>(data: I) -> Vec<Pair> {
    data.map(|s| Pair::from(s.as_bytes())).collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pair {
    Literal(u8),
    Pair(Box<Pair>, Box<Pair>),
}

impl From<&[u8]> for Pair {
    fn from(data: &[u8]) -> Self {
        assert!(data.starts_with(b"["));
        assert!(data.ends_with(b"]"));
        let contents = &data[1..data.len() - 1];
        let split_idx: usize = contents
            .iter()
            .enumerate()
            .scan((0, false), |(count, fin), (idx, x)| {
                if *fin {
                    None
                } else if *x == b'[' {
                    *count += 1;
                    Some(idx)
                } else if *x == b']' {
                    *count -= 1;
                    Some(idx)
                } else if *x == b',' && *count == 0 {
                    *fin = true;
                    Some(idx)
                } else {
                    Some(idx)
                }
            })
            .last()
            .unwrap();
        let (left_bytes, right_bytes) = contents.split_at(split_idx);
        let right_bytes = right_bytes.split_at(1).1;
        let left = if left_bytes.len() == 1 {
            Pair::Literal(left_bytes[0] - b'0')
        } else {
            Pair::from(left_bytes)
        };
        let right = if right_bytes.len() == 1 {
            Pair::Literal(right_bytes[0] - b'0')
        } else {
            Pair::from(right_bytes)
        };
        Pair::Pair(Box::new(left), Box::new(right))
    }
}

impl Add for Pair {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut temp = Pair::Pair(Box::new(self), Box::new(other));
        temp.reduce();
        temp
    }
}

impl<'a> Add<&'a Pair> for &'a Pair {
    type Output = Pair;

    fn add(self, other: Self) -> Self::Output {
        let mut temp = Pair::Pair(Box::new(self.clone()), Box::new(other.clone()));
        temp.reduce();
        temp
    }
}

impl std::iter::Sum<Pair> for Pair {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.reduce(|accum, item| accum + item).unwrap()
    }
}

impl Default for Pair {
    fn default() -> Self {
        Self::Literal(0)
    }
}

impl Pair {
    fn eval(&self) -> u64 {
        match self {
            Pair::Literal(u) => *u as u64,
            Pair::Pair(l, r) => (3 * l.eval()) + (2 * r.eval()),
        }
    }

    fn is_literal(&self) -> bool {
        matches!(self, Pair::Literal(_))
    }

    fn unwrap(&self) -> u8 {
        if let Self::Literal(u) = self {
            *u
        } else {
            panic!();
        }
    }

    fn add_right(&mut self, value: u8) {
        if let Pair::Literal(ref mut u) = self {
            /*
            println!(
                "Explosion value {} being added to rightmost literal {}",
                value, u
            );
            */
            *u += value;
        }
        if let Pair::Pair(_, r) = self {
            r.add_right(value);
        }
    }

    fn add_left(&mut self, value: u8) {
        if let Pair::Literal(ref mut u) = self {
            /*
            println!(
                "Explosion value {} being added to leftmost literal {}",
                value, u
            );
            */
            *u += value;
        }
        if let Pair::Pair(l, _) = self {
            l.add_left(value);
        }
    }

    fn explode_children(&mut self, depth: u8, not_reduced: &mut bool) -> (Option<u8>, Option<u8>) {
        let mut float = (None, None);
        if let Self::Pair(ref mut l, ref mut r) = self {
            let l_float = l.find_exploders(depth + 1, not_reduced);
            if l_float.0.is_some() {
                // This needs to be propagated to the parent to be added to the rightmost
                // descendent of its left child
                float.0 = l_float.0
            }
            if let Some(u) = l_float.1 {
                /*
                println!(
                    "Adding {} to left-most descendent of right child {:?}",
                    u, r
                );
                */
                r.add_left(u);
            }

            let r_float = r.find_exploders(depth + 1, not_reduced);
            if let Some(u) = r_float.0 {
                /*
                println!(
                    "Adding {} to right-most descendent of left child {:?}",
                    u, l
                );
                */
                l.add_right(u);
            }
            if r_float.1.is_some() {
                // This needs to be propagated to the parent to be added to the rightmost
                // descendent of its left child
                float.1 = r_float.1;
            }
        }
        float
    }

    fn explode(&mut self, not_reduced: &mut bool) -> (Option<u8>, Option<u8>) {
        if let Self::Pair(l, r) = self {
            *not_reduced = true;
            let (float_l, float_r) = (l.unwrap(), r.unwrap());
            //println!("Exploding [{}, {}]", float_l, float_r);
            *self = Self::default();
            (Some(float_l), Some(float_r))
        } else {
            (None, None)
        }
    }

    fn find_exploders(&mut self, depth: u8, not_reduced: &mut bool) -> (Option<u8>, Option<u8>) {
        if self.is_literal() {
            return (None, None);
        }

        //println!("Exploding {:?}", self);
        match depth.cmp(&4) {
            std::cmp::Ordering::Less => self.explode_children(depth, not_reduced),
            std::cmp::Ordering::Equal => self.explode(not_reduced),
            std::cmp::Ordering::Greater => {
                unreachable!();
            }
        }
    }

    fn split_children(&mut self, not_reduced: &mut bool) {
        if *not_reduced {
            return;
        }

        if let Pair::Literal(u) = self {
            if *u >= 10 {
                let left = *u / 2;
                let right = (*u / 2) + (*u % 2);
                //println!("Split: {} -> [{}, {}]", u, left, right);
                *self = Pair::Pair(
                    Box::new(Pair::Literal(left)),
                    Box::new(Pair::Literal(right)),
                );
                *not_reduced = true;
                return;
            }
        }

        if let Pair::Pair(l, r) = self {
            l.split_children(not_reduced);
            r.split_children(not_reduced);
        }
    }

    fn reduce(&mut self) {
        let mut not_reduced = true;

        while not_reduced {
            let mut exploded = false;
            self.explode_children(0, &mut exploded);
            if exploded {
                not_reduced = true;
                continue;
            }

            not_reduced = false;

            let mut split = false;
            self.split_children(&mut split);
            not_reduced |= split;
        }
    }
}

pub fn star1(snailpairs: &[Pair]) -> u64 {
    let sum: Pair = snailpairs.iter().cloned().sum();
    sum.eval()
}

pub fn star2(snailpairs: &[Pair]) -> u64 {
    let mut max_magnitude: u64 = 0;
    for a in 0..snailpairs.len() {
        for b in 0..snailpairs.len() {
            if a == b {
                continue;
            }
            max_magnitude = max_magnitude.max((&snailpairs[a] + &snailpairs[b]).eval());
        }
    }
    max_magnitude
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let left = Pair::from("[[[[4,3],4],4],[7,[[8,4],9]]]".as_bytes());
        let right = Pair::from("[1,1]".as_bytes());
        let sum = Pair::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".as_bytes());

        assert_eq!(left + right, sum);
    }

    const SAMPLE_ADDITION_DATA: [&'static str; 10] = [
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
        "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
        "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
        "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
        "[7,[5,[[3,8],[1,4]]]]",
        "[[2,[2,2]],[8,[8,1]]]",
        "[2,9]",
        "[1,[[[9,3],9],[[9,0],[0,7]]]]",
        "[[[5,[7,4]],7],1]",
        "[[[[4,2],2],6],[8,7]]",
    ];

    #[test]
    fn test_addition_sample() {
        let data = parse(SAMPLE_ADDITION_DATA.iter().map(ToString::to_string));
        let sum: Pair = data.iter().cloned().sum();

        assert_eq!(
            sum,
            Pair::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".as_bytes())
        );
    }

    const SAMPLE_MAGNITUDE_DATA: [&'static str; 9] = [
        "[9,1]",
        "[1,9]",
        "[[9,1],[1,9]]",
        "[[1,2],[[3,4],5]]",
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
    ];

    const SAMPLE_MAGNITUDE_SOLUTIONS: [u64; 9] = [29, 21, 129, 143, 1384, 445, 791, 1137, 3488];

    #[test]
    fn test_magnitude() {
        for (pair, sol) in SAMPLE_MAGNITUDE_DATA
            .iter()
            .map(|s| s.as_bytes())
            .map(|b| Pair::from(b))
            .zip(SAMPLE_MAGNITUDE_SOLUTIONS.iter())
        {
            assert_eq!(pair.eval(), *sol, "{:?} => {}", &pair, sol);
        }
    }

    const SAMPLE_DATA: ([&'static str; 10], &'static str, u64) = (
        [
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ],
        "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
        4140,
    );

    #[test]
    fn test_star1() {
        let data = parse(SAMPLE_DATA.0.iter().map(ToString::to_string));
        let sum: Pair = data.iter().cloned().sum();
        let sum_sol: Pair = Pair::from(SAMPLE_DATA.1.as_bytes());

        assert_eq!(sum, sum_sol);
        assert_eq!(sum.eval(), SAMPLE_DATA.2);
    }

    /*
    #[test]
    fn test_star2() {
        for (i, (input, _, output)) in SAMPLE_DATA.iter().enumerate() {
            let data = parse(input);
            assert_eq!(star1(&data), *output);
        }
    }
    */
}
