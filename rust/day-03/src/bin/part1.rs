use regex::Regex;
use std::cmp;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn find_valid_parts(re: &Regex, prev: &str, line: &str, next: &str) -> Vec<i32> {
    let mut result = vec![];
    let nums = re.find_iter(line);
    let bytes = line.as_bytes();
    let last = bytes.len() - 1;
    let filler = vec![b'.'; last + 1];
    let prev_bytes = if prev.is_empty() {
        &filler[..]
    } else {
        prev.as_bytes()
    };
    let next_bytes = if next.is_empty() {
        &filler[..]
    } else {
        next.as_bytes()
    };
    for num in nums {
        let start = num.start();
        let end = num.end();
        if (start > 0 && bytes[start - 1] != b'.')
            || end < last && bytes[end] != b'.'
            || prev_bytes
                [(if start > 0 { start - 1 } else { 0 })..cmp::min(end + 1, prev_bytes.len())]
                .iter()
                .any(|c| *c != b'.')
            || next_bytes
                [(if start > 0 { start - 1 } else { 0 })..cmp::min(end + 1, next_bytes.len())]
                .iter()
                .any(|c| *c != b'.')
        {
            result.push(num.as_str().parse::<i32>().unwrap_or_default())
        }
    }
    result
}

fn sum_part_numbers(lines: Vec<&str>) -> i32 {
    let nums = Regex::new(r"(\d+)").unwrap();
    let mut parts = vec![];
    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            parts.append(&mut find_valid_parts(&nums, "", line, lines[i + 1]));
        } else if i == lines.len() - 1 {
            parts.append(&mut find_valid_parts(&nums, lines[i - 1], line, ""));
        } else {
            parts.append(&mut find_valid_parts(
                &nums,
                lines[i - 1],
                line,
                lines[i + 1],
            ));
        }
    }
    parts.iter().sum()
}

fn part1(input: &str) -> i32 {
    let matrix: Vec<&str> = input.lines().collect();
    sum_part_numbers(matrix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.m
...$.*....
.664.598..",
        );
        assert_eq!(result, 4361);
    }
}
