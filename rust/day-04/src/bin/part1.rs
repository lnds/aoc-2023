use regex::Regex;
use std::collections::HashSet;
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

struct Card {
    win: HashSet<i32>,
    bet: Vec<i32>,
}

impl Card {
    fn new(line: &str) -> Self {
        let p = line.find(':').unwrap_or_default();
        let q = line.find('|').unwrap_or_default();
        let left = &line[p + 1..q];
        let right = &line[q + 1..];
        let re = Regex::new(r"(\d+)").unwrap();
        Card {
            win: re
                .find_iter(left)
                .map(|m| m.as_str().parse::<i32>().unwrap_or_default())
                .collect(),
            bet: re
                .find_iter(right)
                .map(|m| m.as_str().parse::<i32>().unwrap_or_default())
                .collect(),
        }
    }

    fn points(&self) -> i32 {
        let n = self.bet.iter().filter(|b| self.win.contains(b)).count();
        if n == 0 {
            0
        } else {
            1 << (n - 1)
        }
    }
}
fn get_cards(line: &str) -> Card {
    Card::new(line)
}

fn part1(input: &str) -> i32 {
    input.lines().map(get_cards).map(|c| c.points()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 13);
    }
}
