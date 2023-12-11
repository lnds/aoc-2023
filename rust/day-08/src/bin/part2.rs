use num::integer::Integer;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct State {
    left: String,
    right: String,
}

impl State {
    fn new(line: &str) -> Self {
        let mut parts = line[1..line.len() - 1].split(", ");
        State {
            left: parts.next().unwrap().to_string(),
            right: parts.next().unwrap().to_string(),
        }
    }
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let program = lines.next().unwrap().to_string();
    lines.next();
    let mut starts = vec![];
    let machine = lines
        .map(|l| {
            let mut ls = l.split(" = ");
            let key = ls.next().map(|s| s.to_string()).unwrap().clone();
            if key.ends_with('A') {
                starts.push(key.clone())
            }
            let state = State::new(ls.next().unwrap());
            (key, state)
        })
        .collect::<HashMap<String, State>>();

    let mut steps = vec![];
    for key in starts.iter() {
        let mut key = key.clone();
        let mut count = 0;
        for c in program.chars().cycle() {
            if key.ends_with('Z') {
                break;
            }
            let state = machine.get(&key);
            if c == 'L' {
                key = state.unwrap().left.to_string();
            } else {
                key = state.unwrap().right.to_string();
            }
            count += 1;
        }
        steps.push(count);
    }
    steps.iter().fold(1, |acc, n| n.lcm(&acc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, 6);
    }
}
