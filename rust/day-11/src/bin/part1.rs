use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Galaxy {
    row: usize,
    col: usize,
}

fn part1(input: &str) -> usize {
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
    let mut cols = (0..input.find('\n').unwrap_or_default()).collect::<HashSet<usize>>();
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            let mut galaxies = vec![];
            for (col, c) in l.chars().enumerate() {
                if c == '#' {
                    cols.remove(&col);
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
                total_distance += if empty_rows.contains(&row) { 2 } else { 1 };
            }
            for col in usize::min(o.col, g.col)..usize::max(o.col, g.col) {
                total_distance += if cols.contains(&col) { 2 } else { 1 };
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
        let result = part1(
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
        );
        assert_eq!(result, 374);
    }
}
/*
 * 2,5,8
  0123456789012
0|....1........ (0,4)
1|.........2... (1,9)
2|3............ (2,0)
3|.............
4|.............
5|........4.... (5,8)
6|.5........... (6,1)
7|............6 (7,12)
8|.............
9|.............
0|.........7... (10,9)
1|8....9......."(11,0),(11,5)
*/
