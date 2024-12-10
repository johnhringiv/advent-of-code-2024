use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Grid {
    grid: Vec<u32>,
    array_width: usize,
    num_rows: usize,
}

impl Grid {
    fn parse_data<R: BufRead>(reader: R) -> Grid {
        let mut file_iter = reader.lines().peekable();
        let array_width = file_iter.peek().unwrap().as_ref().unwrap().len();
        let grid = file_iter.flat_map(|l| l.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<u32>>();
        let num_rows = grid.len() / array_width;
        Grid { grid, array_width, num_rows}
    }
    
    fn move_pos(&self, pos: usize, coords: (isize, isize)) -> Option<usize> {
        let (x, y) = coords;
        let y_idx = (pos / self.array_width) as isize + y;
        let x_idx = (pos % self.array_width) as isize + x;

        if (x_idx >= 0) && (x_idx < self.array_width as isize) && (y_idx >= 0) && (y_idx < self.num_rows as isize)
        {
            Some(y_idx as usize * self.array_width + x_idx as usize)
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.grid.len()
    }

    fn peek(&self, pos: usize) -> u32 {
        self.grid[pos]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut row_start = 0;
        while row_start < self.len() {
            write!(f, "{}\n", &self.grid[row_start..row_start + self.array_width].iter().map(|d| d.to_string()).collect::<String>())?;
            row_start += self.array_width;
        }
        write!(f, "\n")
    }
}

fn combined(grid: &Grid) -> (usize, usize) {
    let mut trail_index: HashMap<usize, HashSet<Vec<usize>>> = HashMap::new();
    fn find_trails(grid: &Grid, cur_pos: Option<usize>, mut path: Vec<usize>, all_paths: &mut HashMap<usize, HashSet<Vec<usize>>>) {
        if path.len() == 10 {
            all_paths.entry(path[0]).or_insert(HashSet::new()).insert(path.clone());
        } else {
            if let Some(cur_pos) = cur_pos {
                let cur_val = grid.peek(cur_pos);
                let target = path.len() as u32;
                if cur_val == target {
                    path.push(cur_pos);
                    find_trails(grid, grid.move_pos(cur_pos, (-1, 0)), path.clone(), all_paths);
                    find_trails(grid, grid.move_pos(cur_pos, (1, 0)), path.clone(), all_paths);
                    find_trails(grid, grid.move_pos(cur_pos, (0, -1)), path.clone(), all_paths);
                    find_trails(grid, grid.move_pos(cur_pos, (0, 1)), path, all_paths);
                } else { return; }
            }
        }
    }
    for start in 0..grid.len() {
        find_trails(grid, Some(start), vec![], &mut trail_index);
    }
    // get number of unique trails
    let p2 = trail_index.iter().map(|(_, paths)| paths.len()).sum::<usize>();
    // for p1 we want distinct start and end not paths
    let p1 = trail_index.iter().map(|(_, paths)| paths.iter().map(|x| x[x.len() - 1]).collect::<HashSet<usize>>().len()).sum();
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
