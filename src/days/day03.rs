use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1<R: BufRead>(reader: R) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        for (_, [m1, m2]) in re.captures_iter(&line).map(|c| c.extract()) {
            let n1 = m1.parse::<usize>().unwrap();
            let n2 = m2.parse::<usize>().unwrap();
            result += n1 * n2;
        }
    }
    result
}

fn part2<R: BufRead>(reader: R) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();
    let mut result = 0;
    let mut state_do = true;
    for line in reader.lines() {
        let line = line.unwrap();
        for ins in re.captures_iter(&line) {
            match ins.get(0).unwrap().as_str() {
                "do()" => state_do = true,
                "don't()" => state_do = false,
                _ => {
                    if state_do {
                        let n1 = ins.get(1).unwrap().as_str().parse::<usize>().unwrap();
                        let n2 = ins.get(2).unwrap().as_str().parse::<usize>().unwrap();
                        result += n1 * n2;
                    }
                }
            }
        }
    }
    result
}

pub fn solve() -> (usize, usize) {
    let input_file = BufReader::new(File::open("input/03.txt").unwrap());
    let p1 = part1(input_file);
    let input_file = BufReader::new(File::open("input/03.txt").unwrap());
    let p2 = part2(input_file);
    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(161, part1(BufReader::new(TEST.as_bytes())));
    }

    #[test]
    fn test_part2() {
        assert_eq!(48, part2(BufReader::new(TEST.as_bytes())));
    }

    #[test]
    fn test_solve() {
        assert_eq!((169021493, 111762583), solve());
    }
}
