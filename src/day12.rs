use anyhow::Result;
use std::collections::HashSet;
use std::io::BufRead;

const DATA_FILE: &str = "12.txt";

fn solve_recursive(
    maze: &Maze,
    solutions: &mut HashSet<Vec<usize>>,
    path: &[usize],
    has_revisited: bool,
) {
    let last = path.last().unwrap();
    if *last == 1 {
        if !solutions.contains(path) {
            solutions.insert(path.to_vec());
        }
        return;
    }

    for next_node in maze.adjacencies[*last]
        .iter()
        .filter(|n| !path.contains(n) || !maze.minors.contains(n))
    {
        let mut new_path = path.to_vec();
        new_path.push(*next_node);
        solve_recursive(maze, solutions, &new_path, has_revisited);
    }

    if !has_revisited {
        for next_node in maze.adjacencies[*last]
            .iter()
            .filter(|n| path.contains(n) && maze.minors.contains(n))
        {
            let mut new_path = path.to_vec();
            new_path.push(*next_node);
            solve_recursive(maze, solutions, &new_path, true);
        }
    }
}

fn solve_all(maze: &Maze, solutions: &mut HashSet<Vec<usize>>, has_revisited: bool) {
    solve_recursive(maze, solutions, &[0], has_revisited);
}

#[derive(Debug, Clone)]
pub struct Maze {
    adjacencies: Vec<HashSet<usize>>,
    minors: HashSet<usize>,
    name_id_map: Vec<(String, usize)>,
}

impl Default for Maze {
    fn default() -> Self {
        let adjacencies = vec![HashSet::new(), HashSet::new()];
        let minors = HashSet::with_capacity(10);
        let name_id_map = vec![(String::from("start"), 0), (String::from("end"), 1)];
        Maze {
            adjacencies,
            minors,
            name_id_map,
        }
    }
}

impl<'a> std::iter::FromIterator<&'a str> for Maze {
    fn from_iter<I: std::iter::IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut maze = Maze::default();
        for line in iter {
            maze.parse_line(line);
        }
        maze
    }
}

impl std::iter::FromIterator<String> for Maze {
    fn from_iter<I: std::iter::IntoIterator<Item = String>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut maze = Maze::default();
        for line in iter {
            maze.parse_line(&line);
        }
        maze
    }
}

impl Maze {
    fn parse_line(&mut self, line: &str) {
        let (a, b) = line.split_once('-').unwrap();
        self.add_adjacency(a, b);
    }

    fn add_node(&mut self, name: &str) -> usize {
        if let Some(id) = self.get_id_by_name(name) {
            return id;
        }

        let id = self.adjacencies.len();
        self.name_id_map.push((name.to_owned(), id));
        self.adjacencies.push(HashSet::new());

        if name.chars().all(|c| c.is_ascii_lowercase()) {
            self.minors.insert(id);
        }
        id
    }

    fn add_adjacency(&mut self, node_a: &str, node_b: &str) {
        let id_a = self.add_node(node_a);
        let id_b = self.add_node(node_b);
        // start cannot be revisited, so keep it out of the target
        // list of adjacencies
        // end cannot be revisited, so don't add anything its list
        // of targets
        if id_a != 1 && id_b != 0 {
            self.adjacencies[id_a].insert(id_b);
        }
        if id_b != 1 && id_a != 0 {
            self.adjacencies[id_b].insert(id_a);
        }
    }

    fn get_id_by_name(&self, name: &str) -> Option<usize> {
        for (entry_name, id) in &self.name_id_map {
            if name == entry_name {
                return Some(*id);
            }
        }
        None
    }

    /*
    fn get_name_by_id(&self, id: usize) -> Option<String> {
        for (name, entry_id) in &self.name_id_map {
            if id == *entry_id {
                return Some(name.to_owned());
            }
        }
        None
    }
    */
}

pub fn data<P: AsRef<std::path::Path>>(data_dir: P) -> Result<Maze> {
    let data_file = data_dir.as_ref().join(DATA_FILE);
    let data = std::io::BufReader::new(std::fs::File::open(&data_file)?);
    data.lines()
        .map(|s_res| s_res.map_err(|e| e.into()))
        .collect::<Result<Maze>>()
}

pub fn star1(data: &Maze) -> u32 {
    let mut solutions = HashSet::with_capacity(200);
    solve_all(data, &mut solutions, true);
    solutions.len() as u32
}

pub fn star2(data: &Maze) -> u32 {
    let mut solutions = HashSet::with_capacity(400);
    solve_all(data, &mut solutions, false);
    solutions.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: [&'static str; 7] =
        ["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];

    const SAMPLE2: [&'static str; 10] = [
        "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa", "kj-HN",
        "kj-dc",
    ];

    const SAMPLE3: [&'static str; 18] = [
        "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
        "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
    ];

    #[test]
    fn test_star1() {
        let maze1 = SAMPLE1.iter().cloned().collect();
        assert_eq!(star1(&maze1), 10);

        let maze2 = SAMPLE2.iter().cloned().collect();
        assert_eq!(star1(&maze2), 19);

        let maze3 = SAMPLE3.iter().cloned().collect();
        assert_eq!(star1(&maze3), 226);
    }

    #[test]
    fn test_star2() {
        let maze1 = SAMPLE1.iter().cloned().collect();
        assert_eq!(star2(&maze1), 36);

        let maze2 = SAMPLE2.iter().cloned().collect();
        assert_eq!(star2(&maze2), 103);

        let maze3 = SAMPLE3.iter().cloned().collect();
        assert_eq!(star2(&maze3), 3509);
    }
}
