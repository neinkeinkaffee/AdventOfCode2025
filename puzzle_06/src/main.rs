use std::fs::read_to_string;

fn main() {
    println!("Welcome to Puzzle 06!");

    let input = read_to_string("input.txt").unwrap();
    let result = solve_puzzle(&input);

    println!("Result: {result}")
}

fn solve_problem(problem: &Vec<String>) -> u128 {
    let n = problem.len();
    let operator = &problem[n-1];
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

fn parse_input(input: &str) -> Vec<Vec<String>> {
    let mut problems: Vec<Vec<String>> = vec![];

    let lines: Vec<_> = input.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let n_lines = lines.len();
    if n_lines == 0 {
        return problems
    }
    let n_chars = lines[0].len();

    let mut problem: Vec<String> = vec![];
    for j in (0..n_chars).rev() {
        let mut number: String = String::from("");
        for i in 0..n_lines-1 {
            let c = &lines[i][j];
            number.push_str(&c.to_string())
        }
        let number_trimmed = number.trim().to_string();
        if number_trimmed != "" {
            problem.push(number_trimmed);
        }
        let maybe_operator = lines[n_lines-1][j];
        if maybe_operator != ' ' {
            problem.push(maybe_operator.to_string());
            problems.push(problem);
            problem = vec![];
        }
    }

    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";

        assert_eq!(parse_input(input), [
            ["4", "431", "623", "+"],
            ["175", "581", "32", "*"],
            ["8", "248", "369", "+"],
            ["356", "24", "1", "*"],
        ])
    }

    #[test]
    fn test_solve_problem() {
        assert_eq!(solve_problem(&["123", "45", "6", "*"].iter().map(|x| x.to_string()).collect::<Vec<String>>()), 33210);
        assert_eq!(solve_problem(&["328", "64", "98", "+"].iter().map(|x| x.to_string()).collect::<Vec<String>>()), 490);
        assert_eq!(solve_problem(&["51", "387", "215", "*"].iter().map(|x| x.to_string()).collect::<Vec<String>>()), 4243455);
        assert_eq!(solve_problem(&["64", "23", "314", "+"].iter().map(|x| x.to_string()).collect::<Vec<String>>()), 401);
        assert_eq!(solve_problem(&["1", "2", "3", "*"].iter().map(|x| x.to_string()).collect::<Vec<String>>()), 6);
    }

    #[test]
    fn test_solve_puzzle() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";

        assert_eq!(solve_puzzle(input), 3263827)
    }
}