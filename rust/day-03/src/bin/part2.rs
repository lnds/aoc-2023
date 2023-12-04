use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn find_adjacent_parts(re: &Regex, prev: &str, line: &str, next: &str) -> Vec<i64> {
    let mut result = vec![];
    let bytes = line.as_bytes();
    let last = bytes.len() - 1;
    let filler = vec![b'.'; last + 1];
    let prev_line = if prev.is_empty() {
        std::str::from_utf8(&filler[..]).unwrap_or_default()
    } else {
        prev
    };
    let next_line = if next.is_empty() {
        std::str::from_utf8(&filler[..]).unwrap_or_default()
    } else {
        next
    };
    for (i, c) in bytes.iter().enumerate() {
        if c != &b'*' {
            continue;
        }
        let mut top_parts = re
            .find_iter(prev_line)
            .filter(|m| m.start() < i + 2 && i < (m.end() + 1))
            .map(|m| m.as_str().parse::<i64>().unwrap_or_default())
            .collect();

        let mut bottom_parts = re
            .find_iter(next_line)
            .filter(|m| m.start() < i + 2 && i < (m.end() + 1))
            .map(|m| m.as_str().parse::<i64>().unwrap_or_default())
            .collect();
        let mut inline_parts = re
            .find_iter(line)
            .filter(|m| m.end() == i || i + 1 == m.start())
            .map(|m| m.as_str().parse::<i64>().unwrap_or_default())
            .collect();
        let mut gears = vec![];
        gears.append(&mut top_parts);
        gears.append(&mut bottom_parts);
        gears.append(&mut inline_parts);
        if gears.len() >= 2 {
            result.push(gears.iter().product());
        }
    }
    result
}

fn find_gears(lines: Vec<&str>) -> i64 {
    let nums = Regex::new(r"(\d+)").unwrap();
    let mut parts = vec![];
    for (i, line) in lines.iter().enumerate() {
        if line.chars().any(|c| c == '*') {
            parts.append(&mut find_adjacent_parts(
                &nums,
                if i == 0 { "" } else { lines[i - 1] },
                line,
                if i < lines.len() - 1 {
                    lines[i + 1]
                } else {
                    ""
                },
            ))
        }
    }
    parts.iter().sum()
}

fn part2(input: &str) -> i64 {
    let matrix: Vec<&str> = input.lines().collect();
    find_gears(matrix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 467835);
    }
}
