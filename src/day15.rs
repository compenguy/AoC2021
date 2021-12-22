use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::BufRead;

const DATA_FILE: &str = "15.txt";

pub fn data<P: AsRef<std::path::Path>>(data_dir: P) -> impl Iterator<Item = String> {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let data = std::io::BufReader::new(std::fs::File::open(&data_file).unwrap());
    data.lines().map(|s_res| s_res.unwrap())
}

pub fn parse<I: Iterator<Item = String>>(data: I) -> Vec<Vec<u8>> {
    let mut maze: Vec<Vec<u8>> = Vec::with_capacity(100);
    for line in data {
        let row: Vec<u8> = line.bytes().map(|x| x - b'0').collect();
        maze.push(row)
    }
    maze
}

#[derive(Debug, Clone, Default)]
struct Cell {
    weight: u32,
    point: (usize, usize),
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .weight
            .cmp(&self.weight)
            .then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<(usize, usize)> for Cell {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.point == *other
    }
}

impl PartialEq<Cell> for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

impl Eq for Cell {}

impl Cell {
    fn new(point: (usize, usize), weight: u32) -> Self {
        Cell { weight, point }
    }
}

fn find_neighbors(maze: &[Vec<u8>], point: &(usize, usize)) -> Vec<(usize, usize)> {
    let (y, x) = *point;
    let mut neighbors: Vec<(usize, usize)> = Vec::with_capacity(4);

    let min_y = y.saturating_sub(1);
    let min_x = x.saturating_sub(1);
    let max_y = (maze.len() - 1).min(y + 1);
    let max_x = (maze[max_y].len() - 1).min(x + 1);

    if max_y > y {
        neighbors.push((max_y, x));
    }
    if max_x > x {
        neighbors.push((y, max_x));
    }
    if min_y < y {
        neighbors.push((min_y, x));
    }
    if min_x < x {
        neighbors.push((y, min_x));
    }
    neighbors
}

pub fn star1(maze: &[Vec<u8>]) -> u32 {
    let max_y = maze.len() - 1;
    let max_x = maze[max_y].len() - 1;
    let start = (0, 0);

    let mut unvisited: HashSet<(usize, usize)> = (0..=max_y)
        .flat_map(|y| (0..=max_x).map(move |x| (y, x)))
        .collect();
    let mut heap: BinaryHeap<Cell> = BinaryHeap::with_capacity((max_y + 1) * (max_x + 1) / 4);
    heap.push(Cell::default());

    let mut solution_table: HashMap<(usize, usize), Cell> =
        HashMap::with_capacity((max_y + 1) * (max_x + 1));
    solution_table.insert(start, Cell::default());

    //println!("Cells to evaluate: {:?}", heap);
    while let Some(cell) = heap.pop() {
        let score = solution_table[&cell.point].weight;
        unvisited.remove(&cell.point);
        for neighbor in find_neighbors(maze, &cell.point)
            .into_iter()
            .filter(|pt| unvisited.contains(pt))
        {
            //println!("neighbor: {:?}", neighbor);
            let (next_y, next_x) = neighbor;
            let next_weight: u32 = maze[next_y][next_x] as u32;
            let next_score: u32 = score + next_weight;
            let next_cell = Cell::new(neighbor, next_score);
            let solution_cell = Cell::new(cell.point, next_score);
            // Update solution table
            if next_score
                < solution_table
                    .get(&neighbor)
                    .map(|n| n.weight)
                    .unwrap_or(std::u32::MAX)
            {
                solution_table.insert(neighbor, solution_cell);
                heap.push(next_cell);
            }
        }
        //println!("Cells to evaluate: {:?}", heap);
        //println!("Cells to evaluate: {}", unvisited.len());
    }
    if let Some(cell) = solution_table.get(&(max_y, max_x)) {
        //print_solution(&solution_table, cell.point);
        cell.weight
    } else {
        panic!("No solution found");
    }
}

fn multiply_maze(maze: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let inner_row_count = maze.len();
    let inner_col_count = maze[inner_row_count - 1].len();

    let outer_max_y = (inner_row_count * 5) - 1;
    let outer_max_x = (inner_col_count * 5) - 1;

    let mut out: Vec<Vec<u8>> = Vec::with_capacity(outer_max_y);
    for outer_y in 0..=outer_max_y {
        out.push(Vec::with_capacity(outer_max_x));

        let tile_y = (outer_y / inner_row_count) as u8;
        let inner_y = outer_y % inner_row_count;

        for outer_x in 0..=outer_max_x {
            let tile_x = (outer_x / inner_col_count) as u8;
            let inner_x = outer_x % inner_col_count;

            let start_val = maze[inner_y][inner_x];
            let tile_offset = tile_x + tile_y;
            let new_val = ((start_val + tile_offset - 1) % 9) + 1;

            //println!("{} + {} + {} = {:02} => {}", start_val, tile_x, tile_y, start_val + tile_offset, new_val);
            out[outer_y].push(new_val);
        }
    }
    out
}

pub fn star2(maze: &[Vec<u8>]) -> u32 {
    let bigger_maze = multiply_maze(maze);
    star1(&bigger_maze)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: [&'static str; 10] = [
        "1163751742",
        "1381373672",
        "2136511328",
        "3694931569",
        "7463417111",
        "1319128137",
        "1359912421",
        "3125421639",
        "1293138521",
        "2311944581",
    ];

    #[test]
    fn test_star1() {
        let maze = parse(SAMPLE_DATA.iter().map(|r| r.to_string()));
        assert_eq!(star1(&maze), 40);
    }

    #[test]
    fn test_star2() {
        let maze = parse(SAMPLE_DATA.iter().map(|r| r.to_string()));
        let bigger_maze = multiply_maze(&maze);
        assert_eq!(star1(&bigger_maze), 315);
    }
}
