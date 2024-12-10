use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use crate::days::grid::{Grid, ParseData};

fn find_words(grid: &Grid<char>, target_str: &[char], directions: &[(isize, isize)]) -> Vec<Vec<usize>> {
    let mut solutions = vec![];
    for pos in 0..grid.len() {
        for direction in directions {
            let mut current_pos = pos;
            let mut sol = vec![];
            while sol.len() < target_str.len()
                && grid.peek(current_pos) == target_str[sol.len()]
            {
                sol.push(current_pos);
                match grid.move_pos(current_pos, *direction) {
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

fn part1(grid: &Grid<char>) -> usize {
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
    find_words(grid, &target_str, &directions).iter().count()
}

fn part2(grid: &Grid<char>) -> usize {
    let target_str = ['M', 'A', 'S'];
    let directions = [
        (-1, 1),  // upper left
        (1, 1),   // upper right
        (-1, -1), // lower left
        (1, -1),  // lower right
    ];
    let solutions = find_words(grid, &target_str, &directions);
    let mut result = HashMap::new();
    // looking for solutions that share the a position
    for sol in solutions {
        *result.entry(sol[1]).or_insert(0) += 1;
    }
    result.iter().filter(|&(_, &v)| v > 1).count()
}

pub fn solve() -> (usize, usize) {
    let input_file = BufReader::new(File::open("input/04.txt").unwrap());
    let grid: Grid<char> = Grid::parse_data(input_file);
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
