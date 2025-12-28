use regex::Regex;

fn main() {
    println!("Welcome to Puzzle 10!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n";

        assert_eq!(parse_line(input), (vec![false, true, true, false], vec![
            vec![false, false, false, true],
            vec![false, true, false, true],
            vec![false, false, true, false],
            vec![false, false, true, true],
            vec![true, false, true, false],
            vec![true, true, false, false],
        ]))
    }

    fn parse_input(input: &str) -> Vec<(Vec<bool>, Vec<Vec<bool>>)> {
        input.split("\n")
            .filter(|l| !l.is_empty())
            .fold(vec![],|mut acc, l| {
                acc.push(parse_line(l));
                acc
            })
    }

    fn parse_line(line: &str) -> (Vec<bool>, Vec<Vec<bool>>) {
        let button_regex = Regex::new(r"^\((?<toggled>[0-9,]+)\)$").unwrap();

        let mut split = line.split(" ");

        let target: Vec<bool> = split.next().unwrap().chars()
            .filter(|c| { *c == '#' || *c == '.'})
            .map(|c| {
                match c {
                    '#' => true,
                    _ => false
                }
            })
            .collect();

        let buttons: Vec<Vec<bool>> = split
            .map(|r| {
                button_regex.captures(r)
            })
            .filter(|c| c.is_some())
            .map(|c| {
                let mut result = vec![false; target.len()];
                for s in c.unwrap()["toggled"].split(",") {
                    let i: usize = s.parse().unwrap();
                    result[i] = true;
                }
                result
            })
            .collect();

        (target, buttons)
    }
}