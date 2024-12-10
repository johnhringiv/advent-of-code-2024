use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

use crate::days::grid::{Grid, ParseData};

fn combined(grid: &Grid<u32>) -> (usize, usize) {
    let mut trail_index: HashMap<usize, HashSet<Vec<usize>>> = HashMap::new();

    fn find_trails(grid: &Grid<u32>, cur_pos: usize, mut path: Vec<usize>, all_paths: &mut HashMap<usize, HashSet<Vec<usize>>>) {
        if path.len() == 10 {
            all_paths.entry(path[0]).or_insert(HashSet::new()).insert(path[1..].to_owned());
        } else if grid.peek(cur_pos) == path.len() as u32 {
            path.push(cur_pos);
            [(0, -1), (0, 1), (-1, 0), (1, 0)].iter().filter_map(|&dir| grid.move_pos(cur_pos, dir)).for_each(|x| find_trails(grid, x, path.clone(), all_paths));
        }
    }

    for start in 0..grid.len() {
        find_trails(grid, start, Vec::with_capacity(10), &mut trail_index);
    }
    // get number of unique trails
    let p2 = trail_index.values().map(|x| x.len()).sum();
    // for p1 we want distinct start and end not paths
    let p1 = trail_index.values().map(|paths| paths.iter().map(|x| *x.last().unwrap()).collect::<HashSet<usize>>().len()).sum();
    (p1, p2)
}

pub fn solve() -> (usize, usize) {
    let input_file = BufReader::new(File::open("input/10.txt").unwrap());
    let grid = Grid::parse_data(input_file);
    combined(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn test_combined() {
        let input_file = BufReader::new(TEST.as_bytes());
        let grid = Grid::parse_data(input_file);
        assert_eq!(combined(&grid), (36, 81));
    }

    #[test]
    fn test_sol() {
        assert_eq!((822, 1801), solve());
    }
}
