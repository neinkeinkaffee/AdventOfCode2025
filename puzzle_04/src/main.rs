use std::fs::read_to_string;

fn main() {
    println!("Welcome to Puzzle 04!");

    let input = read_to_string("input.txt").unwrap();
    let accessible_rolls = find_accessible(parse_input(input.trim().parse().unwrap()));

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

fn find_accessible(grid: Vec<Vec<bool>>) -> u32 {
    if grid.is_empty() {
        return 0
    }

    let m = grid.len();
    let n = grid[0].len();
    let mut counter = vec![vec![0; n]; m];

    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == OCCUPIED {
                if j > 0 {
                    counter[i][j-1] += 1;
                }
                if j < m-1 {
                    counter[i][j+1] += 1;
                }
                if i > 0 {
                    counter[i-1][j] += 1;
                }
                if i < n-1 {
                    counter[i+1][j] += 1;
                }
                // diagonals
                if j > 0 && i > 0 {
                    counter[i-1][j-1] += 1;
                }
                if j < m-1 && i > 0 {
                    counter[i-1][j+1] += 1;
                }
                if j > 0 && i < n-1 {
                    counter[i+1][j-1] += 1;
                }
                if j < m-1 && i < n-1 {
                    counter[i+1][j+1] += 1;
                }
            }
        }
    }

    let mut result = 0;
    let mut result_grid: String = "".to_string();
    for (i, row) in grid.iter().enumerate() {
        let mut line: String = "".to_string();
        for (j, cell) in row.iter().enumerate() {
            if *cell && counter[i][j] < 4 {
                result += 1;
                line += "x"
            } else {
                if *cell {
                    line += "@"
                } else {
                    line += "."
                }
            }
        }
        result_grid += format!("{line}\n").as_str();
    }
    println!("{result_grid}");

    result
}

#[cfg(test)]
mod test {
    use crate::{find_accessible, parse_input};

    #[test]
    fn test_parse_input() {
        let input = "..@\n\
                           @.@\n\
                           @@@";
        let want = vec!{vec!{false, false, true}, vec!{true, false, true}, vec!{true, true, true}};
        assert_eq!(parse_input(input.to_string()), want);
    }

    #[test]
    fn test_find_accessible() {
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
        assert_eq!(find_accessible(parse_input(input.to_string())), 13);
    }
}