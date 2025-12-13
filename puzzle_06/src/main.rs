use std::fs::read_to_string;

fn main() {
    println!("Welcome to Puzzle 06!");

    let input = read_to_string("input.txt").unwrap();
    let result = solve_puzzle(&input);

    println!("Result: {result}")
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    let mut result: Vec<Vec<&str>> = vec!{};

    let mut rows: Vec<Vec<&str>> = vec!{};
    for line in input.split("\n") {
        if !line.is_empty() {
            rows.push(line.split_whitespace().collect());
        }
    }

    let n_problems;
    if let Some(first_row) = rows.first() {
        n_problems = first_row.len()
    } else {
        return result
    }

    for i in 0..n_problems {
        let problem = rows.iter().map(|row| {
            return row[i]
        }).collect();
        result.push(problem);
    }

    result
}

fn solve_problem(problem: &[&str]) -> u128 {
    let n = problem.len();
    let operator = problem[n-1];
    let numbers = problem[..n-1].iter()
        .filter_map(|s| s.parse::<u128>().ok());
    if operator == "*" {
        numbers.reduce(|a, b| a * b).unwrap()
    } else {
        numbers.reduce(|a, b| a + b).unwrap()
    }
}

fn solve_puzzle(input: &str) -> u128 {
    let problems = parse_input(input);
    problems.iter().map(|p| solve_problem(p)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "123 328  51 64  1\n\
                            45 64  387 23  2\n\
                             6 98  215 314 3\n\
                           *   +   *   +   *";

        assert_eq!(parse_input(input), [
            ["123", "45", "6", "*"],
            ["328", "64", "98", "+"],
            ["51", "387", "215", "*"],
            ["64", "23", "314", "+"],
            ["1", "2", "3", "*"],
        ]);
    }

    #[test]
    fn test_solve_problem() {
        assert_eq!(solve_problem(&["123", "45", "6", "*"]), 33210);
        assert_eq!(solve_problem(&["328", "64", "98", "+"]), 490);
        assert_eq!(solve_problem(&["51", "387", "215", "*"]), 4243455);
        assert_eq!(solve_problem(&["64", "23", "314", "+"]), 401);
        assert_eq!(solve_problem(&["1", "2", "3", "*"]), 6);
    }

    #[test]
    fn test_solve_puzzle() {
        let input = "123 328  51 64  1\n\
                            45 64  387 23  2\n\
                             6 98  215 314 3\n\
                           *   +   *   +   *";

        assert_eq!(solve_puzzle(input), 4277562)
    }
}