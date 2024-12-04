use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Grid {
    grid: Vec<char>,
    array_width: usize,
}

impl Grid {
    fn parse_data<R: BufRead>(reader: R) -> Grid {
        let mut file_iter = reader.lines();
        let mut grid = file_iter
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .collect::<Vec<char>>();
        let array_width = grid.len();
        while let Some(line) = file_iter.next() {
            grid.extend(line.unwrap().chars());
        }
        Grid { grid, array_width }
    }

    fn move_pos(&self, pos: usize, coords: (isize, isize)) -> Option<usize> {
        let (x, y) = coords;
        let x_idx = (pos / self.array_width) as isize + x;
        let y_idx = (pos % self.array_width) as isize + y;
        let num_rows = (self.grid.len() / self.array_width) as isize;

        if (x_idx >= 0) && (x_idx < self.array_width as isize) && (y_idx >= 0) && (y_idx < num_rows)
        {
            Some(x_idx as usize * self.array_width + y_idx as usize)
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.grid.len()
    }

    fn peek(&self, pos: usize) -> char {
        self.grid[pos]
    }

    fn find_words(&self, target_str: &[char], directions: &[(isize, isize)]) -> Vec<Vec<usize>> {
        let mut solutions = vec![];
        for pos in 0..self.len() {
            for direction in directions {
                let mut current_pos = pos;
                let mut sol = vec![];
                while sol.len() < target_str.len()
                    && self.peek(current_pos) == target_str[sol.len()]
                {
                    sol.push(current_pos);
                    match self.move_pos(current_pos, *direction) {
                        Some(new_pos) => current_pos = new_pos,
                        None => break,
                    }
                }
                if sol.len() == target_str.len() {
                    solutions.push(sol);
                }
            }
        }
        solutions
    }
}

fn part1(grid: &Grid) -> usize {
    let target_str = ['X', 'M', 'A', 'S'];
    let directions = [
        (-1, 0),  // left
        (1, 0),   // right
        (0, 1),   // up
        (0, -1),  // down
        (-1, 1),  // upper left
        (1, 1),   // upper right
        (-1, -1), // lower left
        (1, -1),  // lower right
    ];
    grid.find_words(&target_str, &directions).iter().count()
}

fn part2(grid: &Grid) -> usize {
    let target_str = ['M', 'A', 'S'];
    let directions = [
        (-1, 1),  // upper left
        (1, 1),   // upper right
        (-1, -1), // lower left
        (1, -1),  // lower right
    ];
    let solutions = grid.find_words(&target_str, &directions);
    let mut result = HashMap::new();
    // looking for solutions that share the a position
    for sol in solutions {
        *result.entry(sol[1]).or_insert(0) += 1;
    }
    result.iter().filter(|&(_, &v)| v > 1).count()
}

pub fn solve() -> (usize, usize) {
    let input_file = BufReader::new(File::open("input/04.txt").unwrap());
    let grid = Grid::parse_data(input_file);
    let p1 = part1(&grid);
    let p2 = part2(&grid);
    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_part1() {
        let input_file = BufReader::new(TEST.as_bytes());
        let grid = Grid::parse_data(input_file);
        assert_eq!(18, part1(&grid));
    }

    #[test]
    fn test_part2() {
        let input_file = BufReader::new(TEST.as_bytes());
        let grid = Grid::parse_data(input_file);
        assert_eq!(9, part2(&grid));
    }

    #[test]
    fn test_sol() {
        assert_eq!((2378, 1796), solve());
    }
}
