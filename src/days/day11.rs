use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tailcall::tailcall;

#[tailcall]
fn get_num_digits(x: usize, acc: usize) -> usize {
    if x == 0 {
        return acc;
    }
    get_num_digits(x / 10, acc + 1)
}

// idea finish processing one stone completly
#[tailcall]
fn change_stones(stones: &mut Vec<u64>, processed: &mut Vec<u64>, mut changes: u64) -> u64 {
    if changes == 0 {
        return stones.len() as u64
    }
    if !stones.is_empty() {
        let stone = stones.pop().unwrap();
        if stone == 0 {
            processed.push(1);
        } else {
            let num_digits = get_num_digits(stone as usize, 0);
            if num_digits % 2 == 0 {
                let left = stone / 10u64.pow((num_digits / 2) as u32);
                let right = stone % 10u64.pow((num_digits / 2) as u32);
                processed.push(right);
                processed.push(left);
            } else {
                processed.push(stone * 2024);
            }
        }
    } else {
        processed.reverse();
        *stones = processed.clone();
        processed.clear();
        changes -= 1;
    }
    change_stones(stones, processed, changes)
}

fn change_stone(stone: u64, cache: &mut HashMap<(u64, u64), u64>, changes: u64) -> u64 {
    if changes == 0 {
        return 1;
    }
    if let Some(&result) = cache.get(&(stone, changes)) {
        return result;
    }
    let ans = if stone == 0 {
        change_stone(1, cache, changes - 1)
    } else {
        let num_digits = get_num_digits(stone as usize, 0);
        if num_digits % 2 == 0 {
            let left = stone / 10u64.pow((num_digits / 2) as u32);
            let right = stone % 10u64.pow((num_digits / 2) as u32);
            change_stone(right, cache, changes - 1) + change_stone(left, cache, changes - 1)
        } else {
            change_stone(stone * 2024, cache, changes - 1)
        }
    };
    cache.insert((stone, changes), ans);
    ans
}

fn parse_data<R: BufRead>(reader: R) -> Vec<u64> {
    reader.lines().next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>()
}

fn combined(data: &Vec<u64>) -> (u64, u64) {
    let mut cache = HashMap::new();
    let p1: u64 = data.iter().map(|&stone| change_stone(stone, &mut cache, 25)).sum();
    let p2: u64 = data.iter().map(|&stone| change_stone(stone, &mut cache, 75)).sum();
    (p1, p2)
}

pub fn solve() -> (usize, usize) {
    let input_file = BufReader::new(File::open("input/11.txt").unwrap());
    let data = parse_data(input_file);
    let (p1, p2) = combined(&data);
    (p1 as usize, p2 as usize)
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use super::*;

    const TEST: &str = "125 17";

    #[test]
    fn test_part1() {
        let input_file = BufReader::new(TEST.as_bytes());
        let data = parse_data(input_file);
        assert_eq!(change_stones(&mut data.clone(), &mut Vec::with_capacity(data.len()), 6), 22);
        assert_eq!(change_stones(&mut data.clone(), &mut Vec::with_capacity(data.len()), 25), 55312);
    }
    
    #[test]
    fn test_solve() {
        assert_eq!((189167, 225253278506288), solve());
    }
}