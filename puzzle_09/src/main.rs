use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    println!("Welcome to Puzzle 09!");

    let input = parse_input(read_to_string("input.txt").unwrap().as_str());

    let now = Instant::now();
    let max_rectangle = find_largest_rectangle(input.clone());
    let elapsed = now.elapsed();

    println!("Largest rectangular area: {max_rectangle}");
    println!("Elapsed: {:.2?}", elapsed);

    let now_tiled = Instant::now();
    let max_tiled_rectangle = find_largest_tiled_rectangle(input);
    let elapsed_tiled = now_tiled.elapsed();

    println!("Largest rectangular area with only red and green tiles: {max_tiled_rectangle}");
    println!("Elapsed: {:.2?}", elapsed_tiled);
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let mut tuples = vec![];
    for line in input.split("\n").filter(|l| !l.is_empty()) {
        let split: Vec<&str> = line.split(",").collect();
        if split.len() == 2 {
            let x: usize = split[0].parse().unwrap();
            let y: usize = split[1].parse().unwrap();
            tuples.push((x, y));
        }
    }

    tuples
}

fn find_largest_rectangle(input: Vec<(usize, usize)>) -> usize {
    let mut max_rectangle = 0;

    for i in 0..input.len() {
        for j in i..input.len() {
            let x1 = input[i].0;
            let x2 = input[j].0;
            let tiles_x = (max(x1, x2) - min(x1, x2)) + 1;
            let y1 = input[i].1;
            let y2 = input[j].1;
            let tiles_y = (max(y1, y2) - min(y1, y2)) + 1;
            let rectangle = tiles_x * tiles_y;
            if max_rectangle < rectangle {
                max_rectangle = rectangle
            }
        }
    }

    max_rectangle
}

fn find_largest_tiled_rectangle(input: Vec<(usize, usize)>) -> usize {
    let tiles_by_row = map_tiles_by_row(&input);
    let tiles_by_column = map_tiles_by_column(&input);

    let mut max_rectangle = 0;
    for i in 0..input.len() {
        'check_pairs: for j in (i+1)..input.len() {
            let x1 = input[i].0;
            let x2 = input[j].0;
            let y1 = input[i].1;
            let y2 = input[j].1;
            println!("Checking input pair {i} and {j}, ({x1}, {y1}) and ({x2}, {y2})");
            let min_x = min(x1, x2);
            let max_x = max(x1, x2);
            let delta_x = (max_x - min_x) + 1;
            let min_y = min(y1, y2);
            let max_y = max(y1, y2);
            let delta_y = (max_y - min_y) + 1;
            let rectangle = delta_x * delta_y;
            if max_rectangle < rectangle {
                println!("\tCurrent max: {max_rectangle}, new candidate max: {rectangle}");
                if rectangle_contains_red_tiles(min_x, max_x, min_y, max_y, &tiles_by_row) {
                    println!("\tDiscard candidate: found red tile(s) inside rectangle");
                    continue
                }
                // check whether corners are tiled
                if !is_tile((x1, y2), &input, &tiles_by_row, &tiles_by_column) || !is_tile((x2, y1), &input, &tiles_by_row, &tiles_by_column) {
                    println!("\tDiscard candidate: not a rectangle");
                    continue
                }
                // check whether vertical borders are tiled
                for y_k in min_y+1..max_y {
                    if !is_tile((x1, y_k), &input, &tiles_by_row, &tiles_by_column) || !is_tile((x2, y_k), &input, &tiles_by_row, &tiles_by_column) {
                        println!("\tDiscard candidate: not a rectangle");
                        continue 'check_pairs
                    }
                }
                // check whether horizontal borders are tiled
                for x_k in min_x+1..max_x {
                    if !is_tile((x_k, y1), &input, &tiles_by_row, &tiles_by_column) || !is_tile((x_k, y2), &input, &tiles_by_row, &tiles_by_column) {
                        println!("\tDiscard candidate: not a rectangle");
                        continue 'check_pairs
                    }
                }
                println!("\tNew max: {x1}, {y1} and {x2}, {y2} with area {rectangle}");
                max_rectangle = rectangle
            }
        }
    }

    max_rectangle
}

fn map_tiles_by_row(input: &Vec<(usize, usize)>) -> HashMap<usize, (usize, usize)> {
    input.iter()
        .fold(HashMap::new(), |mut acc, (x, y)| {
            acc.entry(*y).or_insert(vec![]).push(x);
            acc
        })
        .iter()
        .fold(HashMap::new(), |mut acc, (k, v)| {
            if v[0] < v[1] {
                acc.insert(*k, (*v[0], *v[1]));
            } else {
                acc.insert(*k, (*v[1], *v[0]));
            }
            acc
        })
}

fn map_tiles_by_column(input: &Vec<(usize, usize)>) -> HashMap<usize, (usize, usize)> {
    input.iter()
        .fold(HashMap::new(), |mut acc, (x, y)| {
            acc.entry(*x).or_insert(vec![]).push(y);
            acc
        })
        .iter()
        .fold(HashMap::new(), |mut acc, (k, v)| {
            if v[0] < v[1] {
                acc.insert(*k, (*v[0], *v[1]));
            } else {
                acc.insert(*k, (*v[1], *v[0]));
            }
            acc
        })
}

fn rectangle_contains_red_tiles(min_x: usize, max_x: usize, min_y: usize, max_y: usize, first_last_per_row: &HashMap<usize, (usize, usize)>) -> bool {
    let inner_red_tiles = first_last_per_row.clone().into_iter()
        .filter(|&(k, _)| (min_y < k) && (k < max_y))
        .filter(|(_, v)| (min_x < v.0 && v.0 < max_x) || (min_x < v.1 && v.1 < max_x))
        .collect::<Vec<_>>();
    inner_red_tiles.len() > 0
}

fn is_tile(p: (usize, usize), input: &Vec<(usize, usize)>, tiles_by_row: &HashMap<usize, (usize, usize)>, tiles_by_column: &HashMap<usize, (usize, usize)>) -> bool {
    if input.contains(&p) {
        return true
    }

    let min_x_before = min_x_before(p, tiles_by_row);
    let max_x_before = max_x_before(p, tiles_by_row);
    let min_y_before = min_y_before(p, tiles_by_column);
    let max_y_before = max_y_before(p, tiles_by_column);
    let min_x_after = min_x_after(p, tiles_by_row);
    let max_x_after = max_x_after(p, tiles_by_row);
    let min_y_after = min_y_after(p, tiles_by_column);
    let max_y_after = max_y_after(p, tiles_by_column);

    if let Some(min_x) = min_x_before.max(min_x_after) {
        if let Some(max_x) = max_x_before.min(max_x_after) {
            if let Some(min_y) = min_y_before.max(min_y_after) {
                if let Some(max_y) = max_y_before.min(max_y_after) {
                    if (min_x <= p.0 && p.0 <= max_x) && (min_y <= p.1 && p.1 <= max_y) {
                        return true
                    }
                }
            }
        }
    };

    false
}

fn min_x_before(p: (usize, usize), first_last_per_row: &HashMap<usize, (usize, usize)>) -> Option<usize> {
    let min_x_before = first_last_per_row.iter()
        .filter(|&(k, v)| *k <= p.1 && !(v.0 < p.0 && v.1 < p.0))
        .map(|(_, v)| v.0)
        .min();
    min_x_before
}

fn min_x_after(p: (usize, usize), first_last_per_row: &HashMap<usize, (usize, usize)>) -> Option<usize> {
    let min_x_after = first_last_per_row.iter()
        .filter(|&(k, v)| p.1 <= *k && !(v.0 < p.0 && v.1 < p.0))
        .map(|(_, v)| v.0)
        .min();
    min_x_after
}

fn max_x_before(p: (usize, usize), first_last_per_row: &HashMap<usize, (usize, usize)>) -> Option<usize> {
    let max_x_before = first_last_per_row.iter()
        .filter(|&(k, v)| *k <= p.1 && !(v.0 > p.0 && v.1 > p.0))
        .map(|(_, v)| v.1)
        .max();
    max_x_before
}

fn max_x_after(p: (usize, usize), first_last_per_row: &HashMap<usize, (usize, usize)>) -> Option<usize> {
    let max_x_after = first_last_per_row.iter()
        .filter(|&(k, v)| p.1 <= *k && !(v.0 > p.0 && v.1 > p.0))
        .map(|(_, v)| v.1)
        .max();
    max_x_after
}

fn min_y_before(p: (usize, usize), first_last_per_column: &HashMap<usize, (usize, usize)>) -> Option<usize> {
    let min_y_before = first_last_per_column.iter()
        .filter(|&(k, v)| *k <= p.0 && !(v.0 < p.1 && v.1 < p.1))
        .map(|(_, v)| v.0)
        .min();
    min_y_before
}

fn min_y_after(p: (usize, usize), first_last_per_column: &HashMap<usize, (usize, usize)>) -> Option<usize> {
    let min_y_after = first_last_per_column.iter()
        .filter(|&(k, v)| p.0 <= *k && !(v.0 < p.1 && v.1 < p.1))
        .map(|(_, v)| v.0)
        .min();
    min_y_after
}

fn max_y_before(p: (usize, usize), first_last_per_column: &HashMap<usize, (usize, usize)>) -> Option<usize> {
    let max_y_before = first_last_per_column.iter()
        .filter(|&(k, v)| *k <= p.0 && !(v.0 > p.1 && v.1 > p.1))
        .map(|(_, v)| v.1)
        .max();
    max_y_before
}

fn max_y_after(p: (usize, usize), first_last_per_column: &HashMap<usize, (usize, usize)>) -> Option<usize> {
    let max_y_after = first_last_per_column.iter()
        .filter(|&(k, v)| p.0 <= *k && !(v.0 > p.1 && v.1 > p.1))
        .map(|(_, v)| v.1)
        .max();
    max_y_after
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "7,1\n\
                           11,1\n\
                           11,7\n\
                           9,7\n\
                           9,5\n\
                           2,5\n\
                           2,3\n\
                           7,3\n";

        assert_eq!(parse_input(input), [
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ]);
    }

    #[test]
    fn test_find_largest_rectangle() {
        let input = vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ];

        assert_eq!(find_largest_rectangle(input), 50);
    }

    #[test]
    fn test_find_largest_tiled_rectangle() {
        assert_eq!(find_largest_tiled_rectangle(read_red_tiles("..............\n\
                                                                      .......#XXX#..\n\
                                                                      .......XXXXX..\n\
                                                                      ..#XXXX#XXXX..\n\
                                                                      ..XXXXXXXXXX..\n\
                                                                      ..#XXXXXX#XX..\n\
                                                                      .........XXX..\n\
                                                                      .........#X#..\n")), 24);

        assert_eq!(find_largest_tiled_rectangle(read_red_tiles("..............\n\
                                                                      .......#XXX#..\n\
                                                                      .......XXXXX..\n\
                                                                      ..#XXXX#XXXX..\n\
                                                                      ..XXXXXXXXXX..\n\
                                                                      ..#XXXXXX#XX..\n\
                                                                      .........XXX..\n\
                                                                      .........#X#..\n\
                                                                      .........#X#..\n")), 24);

        assert_eq!(find_largest_tiled_rectangle(read_red_tiles("..............\n\
                                                                      .......#XXX#..\n\
                                                                      .......XXXXX..\n\
                                                                      .......XXXXX..\n\
                                                                      ..#XXXX#XXXX..\n\
                                                                      ..#XXXXXX#XX..\n\
                                                                      .........XXX..\n\
                                                                      .........#X#..\n\
                                                                      ..............\n")), 21);

        assert_eq!(find_largest_tiled_rectangle(read_red_tiles("....................\n\
                                                                      ...#.......#........\n\
                                                                      ......#..........#..\n\
                                                                      ....................\n\
                                                                      ......#..........#..\n\
                                                                      ....................\n\
                                                                      ...#.......#........\n\
                                                                      ....................\n\
                                                                      ....................\n")), 36);

        assert_eq!(find_largest_tiled_rectangle(read_red_tiles("....................\n\
                                                                      ....#XXXXX#.........\n\
                                                                      ....XXXXXXX.........\n\
                                                                      ....#XXXXXXXXXXX#...\n\
                                                                      ........#XX#XXXXX...\n\
                                                                      ........XXXXXXXXX...\n\
                                                                      ........XX#XXXXX#...\n\
                                                                      ........#XX#........\n\
                                                                      ..#XXXX#............\n\
                                                                      ..XXXXXX............\n\
                                                                      ..#XXXX#............\n\
                                                                      ....................\n")), 27);

    }

    #[test]
    fn test_is_inside_tiled_area() {
        let input = read_red_tiles("..............\n\
                                                    .......#XXX#..\n\
                                                    .......XXXXX..\n\
                                                    ..#XXXX#XXXX..\n\
                                                    ..XXXXXXXXXX..\n\
                                                    ..#XXXXXX#XX..\n\
                                                    .........XXX..\n\
                                                    .........#X#..\n\
                                                    ..............\n");
        let first_last_per_row = map_tiles_by_row(&input);
        let first_last_per_column = map_tiles_by_column(&input);

        assert_eq!(is_tile((8, 1), &input, &first_last_per_row, &first_last_per_column), true);
        assert_eq!(is_tile((7, 7), &input, &first_last_per_row, &first_last_per_column), false);
        assert_eq!(is_tile((11, 1), &input, &first_last_per_row, &first_last_per_column), true);
        assert_eq!(is_tile((9, 3), &input, &first_last_per_row, &first_last_per_column), true);
        assert_eq!(is_tile((2, 1), &input, &first_last_per_row, &first_last_per_column), false);

        let input = read_red_tiles("........................\n\
                                                    ...#XXXXX#..............\n\
                                                    ...XXXXXXX..............\n\
                                                    ...#XXXXXXXXXXX#........\n\
                                                    .......#XX#XXXXX........\n\
                                                    .......XXXXXXXXX........\n\
                                                    .......XX#XXXXX#........\n\
                                                    .......#XX#.............\n\
                                                    .#XXXX#.................\n\
                                                    .XXXXXX.................\n\
                                                    .#XXXX#.................\n\
                                                    ........................\n");
        let first_last_per_row = map_tiles_by_row(&input);
        let first_last_per_column = map_tiles_by_column(&input);

        assert_eq!(is_tile((3, 1), &input, &first_last_per_row, &first_last_per_column), true);
        assert_eq!(is_tile((3, 3), &input, &first_last_per_row, &first_last_per_column), true);
        assert_eq!(is_tile((3, 4), &input, &first_last_per_row, &first_last_per_column), false);
    }

    #[test]
    fn test_min_max_x_y() {
        let input = vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ];
        let first_last_per_row = map_tiles_by_row(&input);
        let first_last_per_column = map_tiles_by_column(&input);

        assert_eq!(min_x_before((9, 3), &first_last_per_row), Some(7));
        assert_eq!(min_x_after((9, 3), &first_last_per_row), Some(2));
        assert_eq!(max_x_before((9, 3), &first_last_per_row), Some(11));
        assert_eq!(max_x_after((9, 3), &first_last_per_row), Some(11));

        assert_eq!(min_y_before((9, 3), &first_last_per_column), Some(1));
        assert_eq!(min_y_after((9, 3), &first_last_per_column), Some(1));
        assert_eq!(max_y_before((9, 3), &first_last_per_column), Some(5));
        assert_eq!(max_y_after((9, 3), &first_last_per_column), Some(7));
    }

    #[test]
    fn test_read_red_tiles() {
        let input = "..............\n\
                           .......#XXX#..\n\
                           .......XXXXX..\n\
                           ..#XXXX#XXXX..\n\
                           ..XXXXXXXXXX..\n\
                           ..#XXXXXX#XX..\n\
                           .........XXX..\n\
                           .........#X#..\n\
                           ..............\n";

        assert_eq!(read_red_tiles(input).sort(), vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ].sort());
    }

    fn read_red_tiles(input: &str) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = vec![];

        for (i, line) in input.split("\n").filter(|l| !l.is_empty()).enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    result.push((j, i));
                }
            }
        }

        result
    }
}