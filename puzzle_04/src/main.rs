use std::fs::read_to_string;

fn main() {
    println!("Welcome to Puzzle 04!");

    let input = read_to_string("input.txt").unwrap();
    let accessible_rolls = count_removable_rolls(parse_input(input.trim().parse().unwrap()));

    println!("Accessible rolls: {accessible_rolls}")
}

const EMPTY: bool = false;
const OCCUPIED: bool = true;

fn parse_input(input: String) -> Vec<Vec<bool>> {
    let mut result: Vec<Vec<bool>> = vec!{};
    if input.len() == 0 {
        return result
    }

    let lines = input.split("\n");
    for line in lines {
        let row = line.chars().map(|c| {
            match c {
                '.' => EMPTY,
                '@' => OCCUPIED,
                _ => panic!("invalid input")
            }
        }).collect::<Vec<bool>>();
        result.push(row)
    }

    result
}

fn count_removable_rolls(grid: Vec<Vec<bool>>) -> u32 {
    match count_adjacent_rolls(&grid) {
        None => { 0 },
        Some(counter) => {
            let mut removable_count = 0;
            for (i, row) in grid.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    if *cell && counter[i][j] < 4 {
                        removable_count += 1;
                    }
                }
            }
            let result_grid = display_removal_potential(grid, counter);
            println!("{result_grid}");

            removable_count
        }
    }
}

fn count_adjacent_rolls(grid: &Vec<Vec<bool>>) -> Option<Vec<Vec<i32>>> {
    if grid.is_empty() {
        return None
    }

    let m = grid.len();
    let n = grid[0].len();
    let mut counter = vec![vec![0; n]; m];

    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == OCCUPIED {
                if j > 0 {
                    counter[i][j - 1] += 1;
                }
                if j < m - 1 {
                    counter[i][j + 1] += 1;
                }
                if i > 0 {
                    counter[i - 1][j] += 1;
                }
                if i < n - 1 {
                    counter[i + 1][j] += 1;
                }
                // diagonals
                if j > 0 && i > 0 {
                    counter[i - 1][j - 1] += 1;
                }
                if j < m - 1 && i > 0 {
                    counter[i - 1][j + 1] += 1;
                }
                if j > 0 && i < n - 1 {
                    counter[i + 1][j - 1] += 1;
                }
                if j < m - 1 && i < n - 1 {
                    counter[i + 1][j + 1] += 1;
                }
            }
        }
    }

    Some(counter)
}

fn display_removal_potential(grid: Vec<Vec<bool>>, counter: Vec<Vec<i32>>) -> String {
    let mut removal_potential: String = "".to_string();
    for (i, row) in grid.iter().enumerate() {
        let mut line: String = "".to_string();
        for (j, cell) in row.iter().enumerate() {
            if *cell && counter[i][j] < 4 {
                line += "x"
            } else {
                if *cell {
                    line += "@"
                } else {
                    line += "."
                }
            }
        }
        removal_potential += format!("{line}\n").as_str();
    }
    removal_potential
}

#[cfg(test)]
mod test {
    use crate::{count_removable_rolls, parse_input};

    #[test]
    fn test_parse_input() {
        let input = "..@\n\
                           @.@\n\
                           @@@";
        let want = vec!{vec!{false, false, true}, vec!{true, false, true}, vec!{true, true, true}};
        assert_eq!(parse_input(input.to_string()), want);
    }

    #[test]
    fn test_count_removable_rolls() {
        let input = "..@@.@@@@.\n\
                           @@@.@.@.@@\n\
                           @@@@@.@.@@\n\
                           @.@@@@..@.\n\
                           @@.@@@@.@@\n\
                           .@@@@@@@.@\n\
                           .@.@.@.@@@\n\
                           @.@@@.@@@@\n\
                           .@@@@@@@@.\n\
                           @.@.@@@.@.";
        assert_eq!(count_removable_rolls(parse_input(input.to_string())), 13);
    }
}