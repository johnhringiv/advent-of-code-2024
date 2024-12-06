use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn combined<R: BufRead>(reader: R) -> (usize, usize) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();
    let mut p2 = 0;
    let mut p1 = 0;
    let mut state_do = true;
    for line in reader.lines() {
        let line = line.unwrap();
        for ins in re.captures_iter(&line) {
            match ins.get(0).unwrap().as_str() {
                "do()" => state_do = true,
                "don't()" => state_do = false,
                _ => {
                    let n1 = ins.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    let n2 = ins.get(2).unwrap().as_str().parse::<usize>().unwrap();
                    if state_do {
                        p2 += n1 * n2;
                    }
                    p1 += n1 * n2;
                }
            }
        }
    }
    (p1, p2)
}

pub fn solve() -> (usize, usize) {
    let input_file = BufReader::new(File::open("input/03.txt").unwrap());
    combined(input_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(161, combined(BufReader::new(TEST.as_bytes())).0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(48, combined(BufReader::new(TEST.as_bytes())).1);
    }

    #[test]
    fn test_solve() {
        assert_eq!((169021493, 111762583), solve());
    }
}
