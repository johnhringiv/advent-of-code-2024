use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
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

    fn pos_to_coords(&self, pos: usize) -> (usize, usize) {
        ((pos % self.array_width), (pos / self.array_width))
    }

    fn coords_to_pos(&self, x: usize, y: usize) -> usize {
        y * self.array_width + x
    }

    fn move_pos(&self, pos: usize, coords: (isize, isize)) -> Option<usize> {
        let (x, y) = coords;
        let y_idx = (pos / self.array_width) as isize + y;
        let x_idx = (pos % self.array_width) as isize + x;
        let num_rows = (self.grid.len() / self.array_width) as isize;

        if (x_idx >= 0) && (x_idx < self.array_width as isize) && (y_idx >= 0) && (y_idx < num_rows)
        {
            Some(y_idx as usize * self.array_width + x_idx as usize)
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
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut row_start = 0;
        while row_start < self.len() {
            write!(f, "{}\n", &self.grid[row_start..row_start + self.array_width].iter().collect::<String>())?;
            row_start += self.array_width;
        }
        write!(f, "\n")
    }
}

fn find_guard(grid: &Grid) -> Option<(usize, (isize, isize))> {
    // finds guard and direction
    let pos = grid.grid.iter().position(|c| ['^', 'v', '<', '>'].contains(c)).unwrap();
    let direction = match grid.peek(pos) {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => return None,
    };
    Some((pos, direction))
}

fn map_obstructions(grid: &Grid) -> (HashMap<usize, HashSet<usize>>, HashMap<usize, HashSet<usize>>) {
    let mut obs_x = HashMap::new();
    let mut obs_y = HashMap::new();

    for (idx, c) in grid.grid.iter().enumerate() {
        if *c == '#' {
            let (x, y) = grid.pos_to_coords(idx);
            obs_x.entry(x).or_insert(HashSet::new()).insert(y);
            obs_y.entry(y).or_insert(HashSet::new()).insert(x);
        }
    }
    (obs_x, obs_y)
}

fn get_next_obs(cur_pos: usize, grid: &Grid, direction: &(isize, isize), obs_x: &HashMap<usize, HashSet<usize>>, obs_y: &HashMap<usize, HashSet<usize>>) -> Option<(usize, (isize, isize))> {
    let (x, y) = grid.pos_to_coords(cur_pos);
    let mut obs_coords = None;
    match direction {
        (0, -1) => {
            if let Some(obs) = obs_x.get(&x) {
                if let Some(next_y) = obs.iter().filter(|&&o| y > o).max() {
                    obs_coords = Some((x, *next_y));
                }
            }
        },
        (0, 1) => {
            if let Some(obs) = obs_x.get(&x) {
                if let Some(next_y) = obs.iter().filter(|&&o| y < o).min() {
                    obs_coords = Some((x, *next_y));
                }
            }
        },
        (-1, 0) => {
            if let Some(obs) = obs_y.get(&y) {
                if let Some(next_x) = obs.iter().filter(|&&o| x > o).max() {
                    obs_coords = Some((*next_x, y));
                }
            }
        },
        (1, 0) => {
            if let Some(obs) = obs_y.get(&y) {
                if let Some(next_x) = obs.iter().filter(|&&o| x < o).min() {
                    obs_coords = Some((*next_x, y));
                }
            }
        },
        _ => {}
    }
    // we need to stop short of the next obs and turn
    // this returns the point before the obs and the new direction
    match obs_coords {
        Some(obs_coords) => {
            let new_direction = change_direction(&direction);
            Some((grid.coords_to_pos((obs_coords.0 as isize - direction.0) as usize, (obs_coords.1 as isize - direction.1) as usize), new_direction))
        },
        _ => {
            None
        },
    }
}

fn change_direction(direction: &(isize, isize)) -> (isize, isize) {
    match direction {
        (0, -1) => (1, 0),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, - 1),
        (1, 0) => (0, 1),
        _ => panic!("Invalid direction"),
    }
}

fn move_guard(grid: &Grid, pos: usize, direction: &(isize, isize)) -> Option<(usize, (isize, isize))> {
    match grid.move_pos(pos, *direction) {
        Some(new_pos) => {
            match grid.peek(new_pos) {
                '#' => {
                    let new_direction = change_direction(&direction);
                    Some((pos, new_direction))
                },
                _ => {
                    Some((new_pos, *direction))
                },
            }
        },
        None => None,
    }
}
fn get_visited(grid: &Grid, start: &Option<(usize, (isize, isize))>) -> HashMap<usize, (isize, isize)> {
    let mut guard_pos = start.clone();
    let mut states = HashMap::new();
    loop {
        match guard_pos {
            Some((pos, direction)) => {
                match states.get(&pos) {
                    Some(_) => {},
                    None => { states.insert(pos, direction); }
                }
                guard_pos = move_guard(&grid, pos, &direction);
            },
            None => break,
        }
    }
    states
}

fn has_loop(grid: &Grid, temp_obs: usize, start_direction: (isize, isize), obs_x: &HashMap<usize, HashSet<usize>>, obs_y: &HashMap<usize, HashSet<usize>>) -> bool {
    let mut visited_obs = HashMap::new();
    let (x, y) = grid.pos_to_coords(temp_obs);
    let mut pos = grid.coords_to_pos((x as isize - start_direction.0) as usize, (y as isize - start_direction.1) as usize);
    let mut direction = start_direction;
    loop {
        match get_next_obs(pos, grid, &direction, &obs_x, &obs_y) {
            Some((obs_pos, new_direction)) => {
                match visited_obs.get(&pos) {
                    Some(count) => {
                        if *count > 2 {
                            return true
                        }
                        visited_obs.insert(pos, count + 1);
                    }
                    None => { visited_obs.insert(pos, 1); }
                }
                pos = obs_pos;
                direction = new_direction;
            },
            None => {
                return false
            },
        }
    }
}

fn combined(grid: &Grid) -> (usize, usize) {
    let start = find_guard(&grid);
    let mut visited = get_visited(&grid, &start);
    let p1 = visited.len();

    let mut sol = 0;
    let (mut obs_x, mut obs_y) = map_obstructions(&grid).to_owned();
    visited.remove(&start.unwrap().0);
    for (pos, dir) in visited.iter() {
        // placing an obs at pos means we need to start at pos - direction
        let (x, y) = grid.pos_to_coords(*pos);
        obs_x.entry(x).or_insert(HashSet::new()).insert(y);
        obs_y.entry(y).or_insert(HashSet::new()).insert(x);

        if has_loop(&grid, *pos, *dir, &obs_x, &obs_y) {
            sol += 1;
        }
        //cleanup obs
        obs_x.get_mut(&x).unwrap().remove(&y);
        obs_y.get_mut(&y).unwrap().remove(&x);
    }
    (p1, sol)
}

pub fn solve() -> (usize, usize) {
    let input_file = std::fs::File::open("input/06.txt").expect("file not found");
    let grid = Grid::parse_data(std::io::BufReader::new(input_file));
    combined(&grid)
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use super::*;

    const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_part1() {
        let input_file = BufReader::new(TEST.as_bytes());
        let grid = Grid::parse_data(input_file);
        assert_eq!(combined(&grid).0, 41);
    }

    #[test]
    fn test_part2() {
        let input_file = BufReader::new(TEST.as_bytes());
        let grid = Grid::parse_data(input_file);
        assert_eq!(combined(&grid).1, 6);
    }

    #[test]
    fn test_sol() {
        assert_eq!((4883, 1655), solve())
    }
}