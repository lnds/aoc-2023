fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn calc_arrangements(pattern: &str, counts: &[usize]) -> usize {
    if counts.is_empty() {
        return if pattern.contains('#') { 0 } else { 1 };
    }
    if pattern.is_empty() {
        return if counts.is_empty() { 1 } else { 0 };
    }

    let mut result = 0;
    if let Some(fc) = pattern.chars().next() {
        if fc == '.' || fc == '?' {
            result += calc_arrangements(&pattern[1..], counts);
        }
        if fc == '#' || fc == '?' {
            if let Some(&first_count) = counts.first() {
                if first_count <= pattern.len()
                    && !pattern[..first_count].contains('.')
                    && (first_count == pattern.len()
                        || pattern.as_bytes().get(first_count) != Some(&b'#'))
                {
                    result += calc_arrangements(
                        pattern.get(first_count + 1..).unwrap_or(""),
                        &counts[1..],
                    );
                }
            }
        }
    }
    result
}

fn count_arrangements(line: &str) -> usize {
    let parts: Vec<&str> = line.split(' ').collect();
    let pattern = parts[0];
    let counts: Vec<usize> = parts[1]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap_or_default())
        .collect();
    let res = calc_arrangements(pattern, &counts);
    println!("{pattern}|{counts:?}={res}");
    res
}

fn part1(input: &str) -> usize {
    input.lines().map(count_arrangements).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1",
        );
        assert_eq!(result, 6);
        let result = part1(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );

        assert_eq!(result, 21);
    }
}
