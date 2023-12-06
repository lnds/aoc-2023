use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut lines = input.lines();
    let time: usize = lines
        .next()
        .map(|l| {
            re.find_iter(l)
                .map(|m| m.as_str())
                .collect::<Vec<&str>>()
                .join("")
                .parse::<usize>()
                .unwrap_or_default()
        })
        .unwrap_or_default();
    let distance: usize = lines
        .next()
        .map(|l| {
            re.find_iter(l)
                .map(|m| m.as_str())
                .collect::<Vec<&str>>()
                .join("")
                .parse::<usize>()
                .unwrap_or_default()
        })
        .unwrap();
    let mut min = 1;
    while (time - min) * min < distance {
        min += 1;
    }
    let mut max = time - 1;
    while (time - max) * max < distance {
        max -= 1;
    }
    max - min + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, 71503);
    }
}
