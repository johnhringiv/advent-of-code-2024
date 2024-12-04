use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_data<R: BufRead>(reader: R) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let row = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        left.push(row[0]);
        right.push(row[1]);
    }
    left.sort();
    right.sort();
    (left, right)
}

fn part1(left: &Vec<i32>, right: &Vec<i32>) -> usize {
    let total = left
        .iter()
        .zip(right.iter())
        .map(|(&l, &r)| (r - l).abs())
        .sum::<i32>();
    total as usize
}

fn part2(left: &Vec<i32>, right: &Vec<i32>) -> usize {
    let total = left
        .iter()
        .map(|&l| right.iter().filter(|&r| *r == l).count() * (l as usize))
        .sum();
    total
}

pub fn solve() -> (usize, usize) {
    let input_file = BufReader::new(File::open("input/01.txt").expect("file not found"));
    let (left, right) = parse_data(input_file);
    let p1 = part1(&left, &right);
    let p2 = part2(&left, &right);
    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_part1() {
        let input_file = BufReader::new(TEST.as_bytes());
        let (left, right) = parse_data(input_file);
        assert_eq!(11, part1(&left, &right));
    }

    #[test]
    fn test_part2() {
        let input_file = BufReader::new(TEST.as_bytes());
        let (left, right) = parse_data(input_file);
        assert_eq!(31, part2(&left, &right));
    }

    #[test]
    fn test_day01() {
        assert_eq!((1879048, 21024792), solve());
    }
}
