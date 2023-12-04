use regex::Regex;
use std::cmp;
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

struct Card {
    points: usize,
}

impl Card {
    fn new(line: &str) -> Self {
        let p = line.find(':').unwrap_or_default();
        let q = line.find('|').unwrap_or_default();
        let left = &line[p + 1..q];
        let right = &line[q + 1..];
        let re = Regex::new(r"(\d+)").unwrap();
        let win: Vec<_> = re
            .find_iter(left)
            .map(|m| m.as_str().parse::<i32>().unwrap_or_default())
            .collect();
        let bet: Vec<_> = re
            .find_iter(right)
            .map(|m| m.as_str().parse::<i32>().unwrap_or_default())
            .collect();
        Card {
            points: bet.iter().filter(|b| win.contains(b)).count(),
        }
    }
}

fn part2(input: &str) -> usize {
    let orig: Vec<_> = input.lines().map(Card::new).collect();
    let n = orig.len();
    let mut copies = vec![1; n];
    for (i, c) in orig.iter().enumerate() {
        for j in i + 1..cmp::min(n + 1, i + c.points + 1) {
            copies[j] += copies[i];
        }
    }
    copies.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 30);
    }
}
