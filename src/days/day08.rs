use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct AntennaMap {
    array_width: isize,
    num_rows: isize,
    antenna: HashMap<char, Vec<usize>>,
}

impl AntennaMap {
    fn parse_data<R: BufRead>(reader: R) -> AntennaMap {
        let mut file_iter = reader.lines();
        let mut antenna = HashMap::new();
        let line = file_iter.next().unwrap().unwrap();
        let array_width = line.len() as isize;
        let mut row = 0;
        line.chars().enumerate().for_each(|(idx, c)| {
            if c != '.' {
                antenna.entry(c).or_insert(Vec::new()).push(idx);
            }
        });
        while let Some(line) = file_iter.next() {
            row += 1;
            line.unwrap().chars().enumerate().for_each(|(idx, c)| {
                if c != '.' {
                    antenna
                        .entry(c)
                        .or_insert(Vec::new())
                        .push(idx + (array_width * row) as usize);
                }
            });
        }
        AntennaMap {
            array_width,
            num_rows: row + 1,
            antenna,
        }
    }

    fn pos_to_coords(&self, pos: usize) -> (isize, isize) {
        let pos = pos as isize;
        ((pos % self.array_width), (pos / self.array_width))
    }

    fn coords_to_pos(&self, x: usize, y: usize) -> usize {
        y * self.array_width as usize + x
    }

    fn is_valid_coords(&self, x: isize, y: isize) -> bool {
        (x >= 0) && (x < self.array_width) && (y >= 0) && (y < self.num_rows)
    }
}

pub fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn combined(grid: &AntennaMap) -> (usize, usize) {
    let mut solutions = HashSet::new();
    let mut p1_solutions = HashSet::new();
    for (_, positions) in &grid.antenna {
        for (idx, &a) in positions.iter().enumerate() {
            for &b in positions[idx + 1..].iter() {
                let (x1, y1) = grid.pos_to_coords(a);
                let (x2, y2) = grid.pos_to_coords(b);

                let dx = x2 - x1;
                let dy = y2 - y1;
                let p1 = (x1 - dx, y1 - dy);
                let p2 = (x2 + dx, y2 + dy);
                for p in [p1, p2].iter() {
                    if grid.is_valid_coords(p.0, p.1) {
                        let antipod = grid.coords_to_pos(p.0 as usize, p.1 as usize);
                        p1_solutions.insert(antipod);
                    }
                }

                // normalize delta for p2
                let g = gcd(dx.abs() as usize, dy.abs() as usize);
                let dx = dx / g as isize;
                let dy = dy / g as isize;

                for delta in [(dx, dy), (-dx, -dy)].iter() {
                    let mut cur_pos = (x1, y1);
                    while grid.is_valid_coords(cur_pos.0, cur_pos.1) {
                        let antipod = grid.coords_to_pos(cur_pos.0 as usize, cur_pos.1 as usize);
                        solutions.insert(antipod);
                        cur_pos = (cur_pos.0 + delta.0, cur_pos.1 + delta.1);
                    }
                }
            }
        }
    }
    (p1_solutions.len(), solutions.len())
}

pub fn solve() -> (usize, usize) {
    let input_file = BufReader::new(File::open("input/08.txt").expect("file not found"));
    let grid = AntennaMap::parse_data(input_file);
    combined(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_part1() {
        let input_file = BufReader::new(TEST.as_bytes());
        let grid = AntennaMap::parse_data(input_file);
        assert_eq!(combined(&grid).0, 14);
    }

    #[test]
    fn test_part2() {
        let input_file = BufReader::new(TEST.as_bytes());
        let grid = AntennaMap::parse_data(input_file);
        assert_eq!(combined(&grid).1, 34);
    }

    #[test]
    fn test_sol() {
        assert_eq!(solve(), (273, 1017));
    }
}
