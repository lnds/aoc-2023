use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn str_to_num(s: &str, last: bool) -> u32 {
    match s {
        "twone" => {
            if last {
                1
            } else {
                2
            }
        }
        "one" | "1" => 1,
        "eightwo" => {
            if last {
                2
            } else {
                8
            }
        }
        "two" | "2" => 2,
        "eighthree" => {
            if last {
                3
            } else {
                8
            }
        }
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "threeight" => {
            if last {
                8
            } else {
                3
            }
        }
        "fiveight" => {
            if last {
                8
            } else {
                5
            }
        }
        "nineight" => {
            if last {
                8
            } else {
                9
            }
        }
        "oneight" => {
            if last {
                8
            } else {
                1
            }
        }
        "eight" | "8" => 8,
        "sevenine" => {
            if last {
                9
            } else {
                7
            }
        }
        "nine" | "9" => 9,
        _ => 0,
    }
}

fn get_num(re: &Regex, input: &str) -> u32 {
    let digits: Vec<_> = re.find_iter(input).map(|m| m.as_str()).collect();
    let a = digits.first().unwrap();
    let a = str_to_num(a, false);
    let b = digits.last().unwrap();
    let b = str_to_num(b, true);
    a * 10 + b
}

fn part2(input: &str) -> u32 {
    let re = Regex::new("oneight|twone|threeight|four|fiveight|six|sevenine|eightwo|eighthree|nineight|one|two|three|five|seven|eight|nine|[1-9]") .unwrap();
    input.lines().map(|l| get_num(&re, &l.to_lowercase())).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }
}
