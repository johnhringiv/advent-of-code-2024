use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_data<R: BufRead>(reader: R) -> (HashMap<usize, HashSet<usize>>, Vec<Vec<usize>>) {
    let mut ordering_rules = HashMap::new();
    let mut pages = Vec::new();
    let mut parse_rules = true;
    for line in reader.lines() {
        let line = line.unwrap();
        if parse_rules {
            if line == "" {
                parse_rules = false;
            } else {
                let rule = line
                    .split('|')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                ordering_rules
                    .entry(rule[0])
                    .or_insert(HashSet::new())
                    .insert(rule[1]);
            }
        } else {
            let row = line
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            pages.push(row);
        }
    }
    (ordering_rules, pages)
}

fn combined(
    ordering_rules: &HashMap<usize, HashSet<usize>>,
    updates: &Vec<Vec<usize>>,
) -> (usize, usize) {
    fn is_valid(ordering_rules: &HashMap<usize, HashSet<usize>>, update: &[usize]) -> bool {
        let mut previous = HashSet::with_capacity(update.len());
        for page in update.iter() {
            if let Some(rules) = ordering_rules.get(page) {
                let res = rules.intersection(&previous).count();
                if res > 0 {
                    return false;
                }
            }
            previous.insert(page.to_owned());
        }
        true
    }
    let mut p1 = 0;
    let mut p2 = 0;
    for update in updates.iter() {
        if !is_valid(ordering_rules, update) {
            let mut fixed = Vec::with_capacity(update.len());
            while fixed.len() != update.len() {
                let mut addition_idx = fixed.len();
                fixed.push(update[fixed.len()]);
                while !is_valid(ordering_rules, &fixed[..addition_idx + 1]) {
                    fixed.swap(addition_idx, addition_idx - 1);
                    addition_idx -= 1;
                }
            }
            p2 += fixed[fixed.len() / 2];
        } else {
            p1 += update[update.len() / 2];
        }
    }
    (p1, p2)
}

pub fn solve() -> (usize, usize) {
    let input_file = BufReader::new(File::open("input/05.txt").unwrap());
    let (ordering_rules, updates) = parse_data(input_file);
    combined(&ordering_rules, &updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_part1() {
        let input_file = BufReader::new(TEST.as_bytes());
        let (ordering_rules, updates) = parse_data(input_file);
        assert_eq!(143, combined(&ordering_rules, &updates).0);
    }

    #[test]
    fn test_part2() {
        let input_file = BufReader::new(TEST.as_bytes());
        let (ordering_rules, updates) = parse_data(input_file);
        assert_eq!(123, combined(&ordering_rules, &updates).1);
    }

    #[test]
    fn test_sol() {
        assert_eq!((6260, 5346), solve());
    }
}
