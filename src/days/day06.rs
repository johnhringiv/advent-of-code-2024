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

    fn pos_to_coords(&self, pos: usize) -> (isize, isize) {
        ((pos % self.array_width) as isize, (pos / self.array_width) as isize)
    }

    fn move_pos(&self, pos: usize, coords: (isize, isize)) -> Option<usize> {
        let (x, y) = coords;
        //todo fix in day04
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

    fn update_cell(&mut self, pos: usize, c: char) {
        self.grid[pos] = c;
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

fn move_guard(grid: &Grid, pos: usize, direction: (isize, isize)) -> Option<(usize, (isize, isize))> {
    match grid.move_pos(pos, direction) {
        Some(new_pos) => {
            match grid.peek(new_pos) {
                '#' => {
                    let new_direction = match direction {
                        (0, -1) => (1, 0),
                        (0, 1) => (-1, 0),
                        (-1, 0) => (0, -1),
                        (1, 0) => (0, 1),
                        _ => panic!("Invalid direction"),
                    };
                    Some((pos, new_direction))
                },
                _ => {
                    Some((new_pos, direction))
                },
            }
        },
        None => None,
    }
    // check if new_pos is an obstacle
}

fn part1(grid: &mut Grid) -> usize {
    let mut guard_pos = find_guard(grid);
    let mut visited = HashSet::new();
    loop {
        match guard_pos {
            Some((pos, direction)) => {
                visited.insert(guard_pos.unwrap().0);
                guard_pos = move_guard(&grid, pos, direction);
            },
            None => break,
        }
    }
    visited.len()
}

fn has_loop(grid: &Grid) -> bool {
    let mut guard_pos = find_guard(grid);
    let mut visited = HashSet::new();
    let mut visited_obs = HashMap::new();
    let mut cur_position = -1;
    let mut step = 0;
    loop {
        match guard_pos {
            Some((pos, direction)) => {
                visited.insert(guard_pos.unwrap().0);
                if pos as isize == cur_position {
                    match visited_obs.get(&pos) {
                        Some(count) => {
                            if *count > 1 {
                                return true
                            }
                            visited_obs.insert(pos, count + 1);
                        }
                        None => { visited_obs.insert(pos, 1); }
                    }
                }
                cur_position = pos as isize;
                guard_pos = move_guard(&grid, pos, direction);
                step += 1;
            },
            None => return false
        }
    }
}

fn part2(grid: &Grid) -> usize {
    // brute force test every psoition visited in part 1
    // slightly better would be to use the direction of the guard must be opsticals on the same x, y
    let mut guard_pos = find_guard(grid);
    let starting_pos = guard_pos.unwrap().0;
    let mut visited = HashSet::new();
    loop {
        match guard_pos {
            Some((pos, direction)) => {
                visited.insert(guard_pos.unwrap().0);
                guard_pos = move_guard(&grid, pos, direction);
            },
            None => break,
        }
    }
    let obstacles = grid.grid.iter().enumerate().filter(|(i, c)| **c == '#').map(|(i, _)| grid.pos_to_coords(i)).collect::<Vec<(isize, isize)>>();
    visited.remove(&starting_pos);
    // loop is 4 turns with same set of elemnts
    // for the box with [bottom left](x1, y1) we need [top left](x1+1, y2) [top right](x2, y2+1), [bottom right](x2-1, y1+1)
    // find obstacles that meet the above criteria
    // for each point in visited let that point be each corner check if existing obsticals meet the criteria

    fn check_bottom_left(bottom_left: (isize, isize), obstacles: &[(isize, isize)]) -> Vec<[(isize, isize); 4]> {
        let mut res = vec![];
        let (x1, y1) = bottom_left;
        let top_left_candidates: Vec<isize>= obstacles.iter().filter(|(obs_x, _)| *obs_x == x1+1).map(|(_, y2)| y2.to_owned()).collect();
        for y2 in top_left_candidates {
            let top_right_candidates = obstacles.iter().filter(|(_, obs_y)| *obs_y == y2+1).map(|(x2, _)| x2.to_owned()).collect::<Vec<isize>>();
            for x2 in top_right_candidates {
                let bottom_right_exists = obstacles.iter().filter(|(obs_x, obs_y)| *obs_x == x2-1 && *obs_y == y1+1).count() > 0;
                if bottom_right_exists {
                    res.push([(x1, y1), (x1+1, y2), (x2, y2+1), (x2-1, y1+1)]);
                }
            }
        }
        res
    }

    fn check_top_left(top_left: (isize, isize), obstacles: &[(isize, isize)]) -> Vec<[(isize, isize); 4]> {
        let mut res = vec![];
        let (x, y2) = top_left;
        let x1 = x-1;
        let x2_from_top_right = obstacles.iter().filter(|(_, obs_y)| *obs_y == y2+1).map(|(x2, _)| x2.to_owned()).collect::<Vec<isize>>();
        for x2 in x2_from_top_right {
            let bottom_left_candidates = obstacles.iter().filter(|(obs_x, obs_y)| *obs_x == x1).map(|(_, y1)| y1.to_owned()).collect::<Vec<isize>>();
            for y1 in bottom_left_candidates {
                let box_coords = vec![(x1, y1), (x2, y2+1), (x2-1, y1+1)];
                if box_coords.iter().map(|coords| obstacles.contains(&coords)).all(|x| x) {
                    res.push([(x1, y1), (x1+1, y2), (x2, y2+1), (x2-1, y1+1)]);
                }
            }
        }
        res
    }

    fn check_top_right(top_left: (isize, isize), obstacles: &[(isize, isize)]) -> Vec<[(isize, isize); 4]> {
        // for the box with [bottom left](x1, y1) we need [top left](x1+1, y2) [top right](x2, y2+1), [bottom right](x2-1, y1+1)
        // with top right x2 and y2 are defined

        let mut res = vec![];
        let (x2, tmp) = top_left;
        let y2 = tmp - 1;

        // we need to check all potential x1, y1
        let x1_from_top_left = obstacles.iter().filter(|(_, obs_y)| *obs_y == y2).map(|(x, _)| x-1.to_owned()).collect::<Vec<isize>>();
        let y1_from_bottom_right: Vec<isize> = obstacles.iter().filter(|(obs_x, obs_y)| *obs_x == x2-1).map(|(_, y)| y-1.to_owned()).collect();

        for &x1 in x1_from_top_left.iter() {
            for &y1 in y1_from_bottom_right.iter() {
                // check for box
                // we exclude the top right corner
                let box_coords = vec![(x1, y1), (x1+1, y2), (x2-1, y1+1)];
                if box_coords.iter().map(|coords| obstacles.contains(&coords)).all(|x| x) {
                    res.push([(x1, y1), (x1+1, y2), (x2, y2+1), (x2-1, y1+1)]);
                }
            }
        }

        res
    }

    fn check_bottom_right(bottom_right: (isize, isize), obstacles: &[(isize, isize)]) -> Vec<[(isize, isize); 4]> {
        // for the box with [bottom left](x1, y1) we need [top left](x1+1, y2) [top right](x2, y2+1), [bottom right](x2-1, y1+1)
        let mut res = vec![];
        let (temp_x, temp_y) = bottom_right;
        let x2 = temp_x + 1;
        let y1 = temp_y - 1;
        let top_right_candidates = obstacles.iter().filter(|(obs_x, obs_y)| *obs_x == x2).map(|(_, y)| y-1).collect::<Vec<isize>>();
        for y2 in top_right_candidates {
            let top_left_candidates = obstacles.iter().filter(|(_, obs_y)| *obs_y == y2).map(|(x, _)| x-1).collect::<Vec<isize>>();
            for x1 in top_left_candidates {
                let bottom_left_exists = obstacles.iter().filter(|(obs_x, obs_y)| *obs_x == x1 && *obs_y == y1).count() > 0;
                if bottom_left_exists {
                    res.push([(x1, y1), (x1+1, y2), (x2, y2+1), (x2-1, y1+1)]);
                }
            }
        }
        res
    }

    // need to filter to hittable points


    let mut solutions = HashSet::new();
    for point in visited {
        // we could try every point as bottom left and check if the other 3 points exist
        //let mut all_points = obstacles.clone();
        //all_points.push(grid.pos_to_coords(point));
        //for p in &all_points {
        //    let res = check_top_left(*p, &all_points);
        //    if res.len() > 0 {
        //        //println!("{:?}", point);
        //        solutions.insert(point);
        //        break
        //    }
        //}
        for check_fun in vec![check_bottom_left, check_top_left, check_top_right, check_bottom_right] {
            let mut all_points = obstacles.clone();
            all_points.push(grid.pos_to_coords(point));
            let res = check_fun(grid.pos_to_coords(point), &all_points);
            if res.len() > 0 {
                solutions.insert(point);
                break
            }
        }
    }
    let mut filtered = 0;
    for sol in solutions {
        let mut my_grid = grid.clone();
        my_grid.update_cell(sol, '#');
        if has_loop(&my_grid) {
            filtered += 1;
        }
    }
    filtered
}

fn brute(grid: Grid) -> usize {
    let mut guard_pos = find_guard(&grid);
    let start_pos = guard_pos.unwrap().0;
    let mut visited = HashSet::new();
    loop {
        match guard_pos {
            Some((pos, direction)) => {
                visited.insert(guard_pos.unwrap().0);
                guard_pos = move_guard(&grid, pos, direction);
            },
            None => break,
        }
    }

    visited.remove(&start_pos);
    let mut sol = 0;
    for pos in visited {
        let mut new_grid = grid.clone();
        new_grid.update_cell(pos, '#');
        if has_loop(&new_grid) {
            sol += 1;
        }
    }
    sol
}

pub fn solve() -> (usize, usize) {
    let input_file = std::fs::File::open("input/06.txt").expect("file not found");
    let mut grid = Grid::parse_data(std::io::BufReader::new(input_file));
    let p1 = part1(&mut grid);
    let p2 = brute(grid);
    //let p2 = part2(&grid);
    (p1, p2)
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
        let mut grid = Grid::parse_data(input_file);
        assert_eq!(part1(&mut grid), 41);
    }

    #[test]
    fn test_part2() {
        let input_file = BufReader::new(TEST.as_bytes());
        let grid = Grid::parse_data(input_file);
        assert_eq!(brute(grid), 6);
    }

    #[test]
    fn test_sol() {
        let (p1, p2) = solve();
        assert_eq!((4883, 1655), solve())
    }
}