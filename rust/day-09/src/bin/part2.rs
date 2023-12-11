fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn predict(nums: Vec<i64>) -> i64 {
    let first = nums[0];
    let diffs: Vec<i64> = nums.windows(2).map(|w| w[1] - w[0]).collect();
    // diffs.len() = last_pos
    if diffs.iter().all(|n| n == &0) {
        first
    } else {
        first - predict(diffs)
    }
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect())
        .map(predict)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, 2);
    }
}
