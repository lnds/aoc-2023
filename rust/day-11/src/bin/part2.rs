use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input, 1_000_000);
    dbg!(output);
}

#[derive(Debug)]
struct Galaxy {
    row: usize,
    col: usize,
}

fn part2(input: &str, expansion: usize) -> usize {
    let empty_rows: HashSet<_> = input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            if l.chars().all(|c| c == '.') {
                Some(row)
            } else {
                None
            }
        })
        .collect();
    let mut empty_cols = (0..input.find('\n').unwrap_or_default()).collect::<HashSet<usize>>();
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            let mut galaxies = vec![];
            for (col, c) in l.chars().enumerate() {
                if c == '#' {
                    empty_cols.remove(&col);
                    galaxies.push(Galaxy { row, col })
                }
            }
            galaxies
        })
        .collect::<Vec<_>>();
    let mut total_distance = 0;
    for (i, g) in galaxies.iter().enumerate() {
        for o in galaxies[0..i].iter() {
            for row in usize::min(o.row, g.row)..usize::max(o.row, g.row) {
                total_distance += if empty_rows.contains(&row) {
                    expansion
                } else {
                    1
                };
            }
            for col in usize::min(o.col, g.col)..usize::max(o.col, g.col) {
                total_distance += if empty_cols.contains(&col) {
                    expansion
                } else {
                    1
                };
            }
        }
    }
    total_distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
            10,
        );
        assert_eq!(result, 1030);
        let result = part2(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
            100,
        );
        assert_eq!(result, 8410);
    }
}
