use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

fn parse_data<R: BufRead>(reader: R) -> VecDeque<(u64, u64, u64)> {
    let line = reader.lines().next().unwrap().unwrap().chars().collect::<Vec<char>>();
    let mut data = VecDeque::new();
    let mut idx = 0;
    while idx < line.len() {
        let file_blocks = line[idx].to_digit(10).unwrap() as u64;
        let free_space = if idx + 1 < line.len() {
            line[idx + 1].to_digit(10).unwrap() as u64
        } else {
            0
        };
        data.push_back(((idx / 2) as u64, file_blocks, free_space));
        idx += 2;
    }
    data
}

fn part1(mut data: VecDeque<(u64, u64, u64)>) -> usize {
    let mut sol = Vec::new();
    let mut front_free = 0;
    while data.len() > 0 {
        if front_free == 0 { // place the front blocks
            let (front_id, front_blocks, new_free) = data.pop_front().unwrap();
            sol.extend(vec![front_id; front_blocks as usize]);
            front_free = new_free;
        } else {
            let (back_id, mut back_blocks, back_free) = data.pop_back().unwrap();
            let blocks_to_place = back_blocks.min(front_free);
            sol.extend(vec![back_id; blocks_to_place as usize]);
            front_free -= blocks_to_place;
            back_blocks -= blocks_to_place;
            if back_blocks > 0 {
                data.push_back((back_id, back_blocks, back_free));
            }
        }
    }
    sol.iter().enumerate().map(|(idx, &val)| idx * val as usize).sum::<usize>()
}

// p2 requires u64 should update the template to use u64
fn part2(mut data: VecDeque<(u64, u64, u64)>) -> u64 {
    let mut attempted = Vec::new();
    let mut processed = vec![false; data.len()];
    processed[0] = true;
    while !processed.iter().all(|&x| x) {
        
        //start by getting the back element
        let (back_id, back_blocks, back_free) = data.pop_back().unwrap();
        
        // if the back element has already been processed, skip it and add to attempted
        // This doesn't seem to matter for my input but would prevent edge cases where a second move is possible
        if processed[back_id as usize] {
            attempted.push((back_id, back_blocks, back_free));
            continue;
        }

        // walk forward through the data until we find a front element that can fit the back blocks
        let mut temp = VecDeque::new();
        let mut placed = false;
        while data.len() > 0 {
            let (front_id, front_blocks, front_free) = data.pop_front().unwrap();
            if !placed && back_blocks <= front_free { // place the back blocks here
                temp.push_back((front_id, front_blocks, 0));
                temp.push_back((back_id, back_blocks, front_free - back_blocks));
                placed = true;
            } else {
                temp.push_back((front_id, front_blocks, front_free));
            }
        }
        if !placed {
            attempted.push((back_id, back_blocks, back_free));
        } else if let Some((t1, t2, t3)) = temp.pop_back() {
                temp.push_back((t1, t2, t3 + back_free + back_blocks));
        }
        processed[back_id as usize] = true;
        data = temp.clone();
    }
    data.extend(attempted.iter().rev());
    let mut p2 = 0 ;
    let mut idx = 0;
    for (id, mut blocks, free) in data {
        while blocks > 0 {
            p2 += idx * id;
            blocks -= 1;
            idx += 1;
        }
        idx += free;
    };
    p2
}

pub fn solve() -> (usize, usize) {
    let input_file = BufReader::new(File::open("input/09.txt").expect("file not found"));
    let data = parse_data(input_file);
    let p1 = part1(data.clone());
    let p2 = part2(data);
    (p1, p2 as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        let input_file = BufReader::new(TEST.as_bytes());
        let data = parse_data(input_file);
        assert_eq!(part1(data), 1928);
        //assert_eq!(11, part1(&left, &right));
    }

    #[test]
    fn test_part2() {
        let input_file = BufReader::new(TEST.as_bytes());
        let data = parse_data(input_file);
        assert_eq!(2858, part2(data));
    }

    #[test]
    fn test_solve() {
        assert_eq!((6386640365805, 6423258376982), solve());
    }
}
