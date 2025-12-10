use std::fs::read_to_string;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Welcome to Puzzle 04!");

    let input = read_to_string("input.txt").unwrap();
    let removed_rolls = count_total_removable(input);

    println!("Removed rolls: {removed_rolls}")
}

fn count_total_removable(input: String) -> u32 {
    let mut grid = parse_input(input.trim().parse().unwrap());
    let mut accessible_rolls = count_removable_rolls(&grid);
    let mut removed_rolls = 0;
    while accessible_rolls > 0 {
        removed_rolls += accessible_rolls;
        grid = remove_removable_rolls(&grid);
        accessible_rolls = count_removable_rolls(&grid);
        println!("Accessible rolls: {accessible_rolls}");
    }
    removed_rolls
}

const EMPTY: bool = false;
const OCCUPIED: bool = true;

fn parse_input(input: String) -> Vec<Vec<bool>> {
    let mut result: Vec<Vec<bool>> = vec![];
    if input.len() == 0 {
        return result;
    }

    let lines = input.split("\n");
    for line in lines {
        let row = line
            .chars()
            .map(|c| match c {
                '.' => EMPTY,
                '@' => OCCUPIED,
                _ => panic!("invalid input"),
            })
            .collect::<Vec<bool>>();
        result.push(row)
    }

    result
}

fn remove_removable_rolls(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    if let Some(counter) = count_adjacent_rolls(grid) {
        let m = grid.len();
        let n = grid[0].len();
        let mut new_grid = vec![vec![false; n]; m];
        for (i, row) in grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell && counter[i][j] < 4 {
                    new_grid[i][j] = false;
                } else {
                    new_grid[i][j] = *cell;
                }
            }
        }
        new_grid
    } else {
        vec![]
    }
}

fn count_removable_rolls(grid: &Vec<Vec<bool>>) -> u32 {
    if let Some(counter) = count_adjacent_rolls(grid) {
        let mut removable_count = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell && counter[i][j] < 4 {
                    removable_count += 1;
                }
            }
        }
        let result_grid = display_removal_potential(grid, &counter);
        println!("\r{result_grid}");
        sleep(Duration::from_millis(200));

        removable_count
    } else {
        0
    }
}

fn count_adjacent_rolls(grid: &Vec<Vec<bool>>) -> Option<Vec<Vec<i32>>> {
    if grid.is_empty() {
        return None;
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

fn display_removal_potential(grid: &Vec<Vec<bool>>, counter: &Vec<Vec<i32>>) -> String {
    let mut removal_potential: String = "".to_string();
    for (i, row) in grid.iter().enumerate() {
        let mut line: String = "".to_string();
        for (j, cell) in row.iter().enumerate() {
            if *cell && counter[i][j] < 4 {
                line += "x"
            } else {
                if *cell { line += "@" } else { line += "." }
            }
        }
        removal_potential += format!("{line}\n").as_str();
    }
    removal_potential
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "..@\n\
                           @.@\n\
                           @@@";
        let want = vec![
            vec![false, false, true],
            vec![true, false, true],
            vec![true, true, true],
        ];
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
        let mut grid = parse_input(input.to_string());
        assert_eq!(count_removable_rolls(&grid), 13);
        grid = remove_removable_rolls(&grid);
        assert_eq!(count_removable_rolls(&grid), 12);
        grid = remove_removable_rolls(&grid);
        assert_eq!(count_removable_rolls(&grid), 7);
        grid = remove_removable_rolls(&grid);
        assert_eq!(count_removable_rolls(&grid), 5);
        grid = remove_removable_rolls(&grid);
        assert_eq!(count_removable_rolls(&grid), 2);
        grid = remove_removable_rolls(&grid);
        assert_eq!(count_removable_rolls(&grid), 1);
        grid = remove_removable_rolls(&grid);
        assert_eq!(count_removable_rolls(&grid), 1);
        grid = remove_removable_rolls(&grid);
        assert_eq!(count_removable_rolls(&grid), 1);
        grid = remove_removable_rolls(&grid);
        assert_eq!(count_removable_rolls(&grid), 1);
    }

    #[test]
    fn test_total_removable_rolls() {
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
        assert_eq!(count_total_removable(input.to_string()), 43);
    }
}
