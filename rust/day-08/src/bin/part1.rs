use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
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

fn part1(input: &str) -> i32 {
    let mut lines = input.lines();
    let program = lines.next().unwrap().to_string();
    lines.next();
    let machine = lines
        .map(|l| {
            let mut ls = l.split(" = ");
            let key = ls.next().map(|s| s.to_string()).unwrap().clone();
            let state = State::new(ls.next().unwrap());
            (key, state)
        })
        .collect::<HashMap<String, State>>();

    let mut steps = 0;
    let mut key = "AAA".to_string();
    let last = "ZZZ".to_string();
    // println!("first: {}, last: {}", key, last);
    // println!("machine: {:#?}", machine);
    for c in program.chars().cycle() {
        if key == last {
            return steps;
        }
        // println!("key: {}, c: {}", key, c);
        let state = machine.get(&key);
        // println!("state: {:?}", state);
        if c == 'L' {
            key = state.unwrap().left.to_string();
        } else {
            key = state.unwrap().right.to_string();
        }
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, 2);
        let result = part1(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ",
        );
        assert_eq!(result, 6);
    }
}
