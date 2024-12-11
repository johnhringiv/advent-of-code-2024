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

fn combined<R: BufRead>(reader: R) -> (u64, u64) {
    let data = reader.lines().next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let mut cache = HashMap::new();
    let p1: u64 = data.iter().map(|&stone| change_stone(stone, &mut cache, 25)).sum();
    let p2: u64 = data.iter().map(|&stone| change_stone(stone, &mut cache, 75)).sum();
    (p1, p2)
}

pub fn solve() -> (usize, usize) {
    let input_file = BufReader::new(File::open("input/11.txt").unwrap());
    let (p1, p2) = combined(input_file);
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
        assert_eq!(combined(input_file).0, 55312);
    }
    
    #[test]
    fn test_solve() {
        assert_eq!((189167, 225253278506288), solve());
    }
}