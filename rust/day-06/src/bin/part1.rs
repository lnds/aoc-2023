use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut lines = input.lines();
    let times: Vec<i32> = lines
        .next()
        .map(|l| {
            re.find_iter(l)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .collect()
        })
        .unwrap();
    let distances: Vec<_> = lines
        .next()
        .map(|l| {
            re.find_iter(l)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .collect()
        })
        .unwrap();
    let counts: Vec<_> = times
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let d = distances[i];
            (1..*t).filter(|k| k * (t - k) > d).count()
        })
        .collect();
    counts.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, 288);
    }
}
