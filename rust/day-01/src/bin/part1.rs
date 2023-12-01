fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn get_num(input: &str) -> u32  {
    let digits: Vec<char> = input.chars().filter(|c| c.is_digit(10)).collect();
    if digits.len() == 0 {
        return 0 
    }
    let n = digits[0].to_digit(10).unwrap_or(0);
        n*10+digits.last().unwrap().to_digit(10).unwrap_or(0)
}

fn part1(input: &str) -> u32 {
    input.lines().map(get_num).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");
        assert_eq!(result, 142);
    }
}
