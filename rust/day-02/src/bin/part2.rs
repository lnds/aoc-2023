fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct Game {
    red: Vec<i32>,
    green: Vec<i32>,
    blue: Vec<i32>,
}

fn line_to_game(line: &str) -> Game {
    let mut p = line.split(':');
    let mut game = Game {
        red: vec![],
        green: vec![],
        blue: vec![],
    };
    p.next();
    let rest = p.next().unwrap_or_default();
    let p = rest.split(';');
    for action in p {
        let q = action.split(',');
        for color in q {
            let color = color.trim();
            if color.ends_with("blue") {
                game.blue.push(
                    (color[0..color.len() - 5])
                        .parse::<i32>()
                        .unwrap_or_default(),
                );
            } else if color.ends_with("red") {
                game.red.push(
                    (color[0..color.len() - 4])
                        .parse::<i32>()
                        .unwrap_or_default(),
                );
            } else if color.ends_with("green") {
                game.green.push(
                    (color[0..color.len() - 6])
                        .parse::<i32>()
                        .unwrap_or_default(),
                );
            }
        }
    }
    game
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(line_to_game)
        .map(|g| {
            g.red.iter().max().unwrap()
                * g.blue.iter().max().unwrap()
                * g.green.iter().max().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286);
    }
}
