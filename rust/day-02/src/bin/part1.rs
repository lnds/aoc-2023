fn main() {
    let input = include_str!("./input1.txt");
    let target = "Game 0: 12 red , 13 green , 14 blue";
    let output = part1(target, input);
    dbg!(output);
}

#[derive(Debug)]
struct Game {
    id: i32,
    red: Vec<i32>,
    green: Vec<i32>,
    blue: Vec<i32>,
}

fn line_to_game(line: &str) -> Game {
    let mut p = line.split(':');
    let id = p
        .next()
        .map(|s| i32::from_str_radix(&s[5..], 10).unwrap_or_default())
        .unwrap_or_default();
    let mut game = Game {
        id: id,
        red: vec![],
        green: vec![],
        blue: vec![],
    };
    let rest = p.next().unwrap_or_default();
    let mut p = rest.split(';');
    while let Some(action) = p.next() {
        let mut q = action.split(',');
        while let Some(color) = q.next() {
            let color = color.trim();
            if color.ends_with("blue") {
                game.blue
                    .push(i32::from_str_radix(&color[0..color.len() - 5], 10).unwrap_or_default());
            } else if color.ends_with("red") {
                game.red
                    .push(i32::from_str_radix(&color[0..color.len() - 4], 10).unwrap_or_default());
            } else if color.ends_with("green") {
                game.green
                    .push(i32::from_str_radix(&color[0..color.len() - 6], 10).unwrap_or_default());
            }
        }
    }
    println!("{} {:?}", line, game);
    game
}

fn part1(target: &str, input: &str) -> i32 {
    let g0 = line_to_game(target);
    let red = g0.red[0];
    let blue = g0.blue[0];
    let green = g0.green[0];
    input
        .lines()
        .map(line_to_game)
        .filter(|g| {
            g.red.iter().all(|r| r <= &red)
                && g.blue.iter().all(|b| b <= &blue)
                && g.green.iter().all(|g| g <= &green)
        })
        .map(|g| g.id)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Game 0: 12 red, 13 green, 14 blue",
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8);
    }
}
