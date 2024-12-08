use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_data<R: BufRead>(reader: R) -> Vec<(usize, Vec<usize>)> {
    let mut data = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let (target_str, rest) = line.split_once(": ").unwrap();
        let target = target_str.parse::<usize>().unwrap();
        let vals = rest
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        data.push((target, vals));
    }
    data
}

fn concat(a: usize, b: usize) -> usize {
    fn get_num_digits(x: usize, acc: usize) -> usize {
        if x == 0 {
            return acc;
        }
        get_num_digits(x / 10, acc + 1)
    }
    a * 10usize.pow(get_num_digits(b, 0) as u32) + b
}

fn combined(data: &Vec<(usize, Vec<usize>)>) -> (usize, usize) {
    fn aux_p1(target: usize, vals: &[usize], acc: usize) -> bool {
        if vals.is_empty() {
            return acc == target;
        }
        if acc > target {
            return false;
        }
        aux_p1(target, &vals[1..], acc + vals[0]) || aux_p1(target, &vals[1..], acc * vals[0])
    }

    fn aux_p2(target: usize, vals: &[usize], acc: usize) -> bool {
        if vals.is_empty() {
            return acc == target;
        }
        if acc > target {
            return false;
        }
        aux_p2(target, &vals[1..], acc + vals[0])
            || aux_p2(target, &vals[1..], acc * vals[0])
            || aux_p2(target, &vals[1..], concat(acc, vals[0]))
    }
    let mut p1 = 0;
    let mut p2 = 0;
    for (target, vals) in data {
        if aux_p1(*target, &vals[1..], vals[0]) {
            p1 += target;
            p2 += target;
        } else if aux_p2(*target, &vals[1..], vals[0]) {
            p2 += target;
        }
    }
    (p1, p2)
}

pub fn solve() -> (usize, usize) {
    let input_file = BufReader::new(File::open("input/07.txt").expect("file not found"));
    let data = parse_data(input_file);
    combined(&data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_part1() {
        let input_file = BufReader::new(TEST.as_bytes());
        let data = parse_data(input_file);
        assert_eq!(combined(&data).0, 3749);
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(1, 2), 12);
        assert_eq!(concat(12, 30), 1230);
    }

    #[test]
    fn test_part2() {
        let input_file = BufReader::new(TEST.as_bytes());
        let data = parse_data(input_file);
        assert_eq!(combined(&data).1, 11387);
    }

    #[test]
    fn test_solve() {
        assert_eq!((2501605301465, 44841372855953), solve());
    }
}
