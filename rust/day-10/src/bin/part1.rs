use std::{collections::HashMap, vec};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

struct Path {
    x: Vec<isize>,
    y: Vec<isize>,
    chart: Vec<Vec<char>>,
    directions: HashMap<char, Vec<isize>>,
}

impl Path {
    fn new(chart: Vec<Vec<char>>, directions: HashMap<char, Vec<isize>>) -> Self {
        Path {
            x: vec![],
            y: vec![],
            chart,
            directions,
        }
    }

    fn add(&mut self, x: isize, y: isize) {
        self.x.push(x);
        self.y.push(y);
    }

    fn len(&self) -> usize {
        self.x.len() / 2
    }

    fn find_start(&self) -> Vec<isize> {
        for (y, v) in self.chart.iter().enumerate() {
            for (x, c) in v.iter().enumerate() {
                if c == &'S' {
                    return vec![x as isize, y as isize];
                }
            }
        }
        vec![-1, -1]
    }

    fn pos_is_start(&self, pos: &[isize]) -> bool {
        self.tile_at(pos) == 'S'
    }

    fn tile_at(&self, pos: &[isize]) -> char {
        let x = pos[0] as usize;
        let y = pos[1] as usize;
        self.chart[y][x]
    }

    fn can_move_to(&self, pos: &[isize], tiles: &str) -> bool {
        let cx = self.x.last().unwrap();
        let cy = self.y.last().unwrap();
        let nx = *cx + pos[0];
        let ny = *cy + pos[1];
        if nx < 0 || ny < 0 || nx as usize == self.chart[0].len() || ny as usize == self.chart.len()
        {
            false
        } else {
            let tile = self.tile_at(&[nx, ny]);
            tiles.contains(tile)
        }
    }

    fn find_next_pos(&self) -> Vec<isize> {
        let cx = self.x.last().unwrap();
        let cy = self.y.last().unwrap();
        let tile = self.chart[*cy as usize][*cx as usize];
        let px = self.x[self.x.len() - 2];
        let py = self.y[self.y.len() - 2];
        let dir = self.directions.get(&tile).unwrap();
        if dir[0] == 0 {
            if py == cy + 1 {
                // up
                vec![*cx, cy - 1]
            } else {
                vec![*cx, cy + 1] // down
            }
        } else if dir[1] == 0 {
            if px == cx + 1 {
                vec![cx - 1, *cy] // backwards
            } else {
                vec![cx + 1, *cy] // forward
            }
        } else if px == *cx {
            vec![cx + dir[0], *cy]
        } else {
            vec![*cx, cy + dir[1]]
        }
    }
}

fn part1(input: &str) -> usize {
    let directions: HashMap<char, Vec<isize>> = HashMap::from([
        ('|', vec![0, 1]),
        ('-', vec![1, 0]),
        ('L', vec![1, -1]),
        ('J', vec![-1, -1]),
        ('7', vec![-1, 1]),
        ('F', vec![1, 1]),
    ]);
    let pipes = input.lines().map(|l| l.chars().collect()).collect();
    let mut path = Path::new(pipes, directions);
    let start = path.find_start();
    path.add(start[0], start[1]);
    if path.can_move_to(&[0, -1], "|F7") {
        path.add(start[0], start[1] - 1);
    } else if path.can_move_to(&[1, 0], "-J7") {
        path.add(start[0] + 1, start[1]);
    }
    loop {
        let next = path.find_next_pos();
        if path.pos_is_start(&next) {
            // loop
            break;
        }
        path.add(next[0], next[1]);
    }
    path.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        );
        assert_eq!(result, 4);
        let result = part1(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(result, 4);
        let result = part1(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(result, 8);
    }
}
