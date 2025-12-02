use regex::Regex;
use std::fs::read_to_string;

fn main() {
    println!("Welcome to Puzzle 01!");

    let lines = read_to_string("input.txt").unwrap().lines().map(String::from).collect();

    let start_state = 50;
    let nil_count = count_nils(start_state, lines);

    println!("Nil count: {nil_count}")
}

fn count_nils(start_state: i32, lines: Vec<String>) -> i32 {
    let mut nil_count = 0;
    let mut next_state = start_state;
    for line in lines {
        next_state = add_modulo_100(next_state, parse_cmd(line.trim()));
        if next_state == 0 {
            nil_count += 1
        }
    }
    nil_count
}

fn parse_cmd(cmd: &str) -> i32 {
    let re = Regex::new(r"^(?<direction>[R|L])(?<num>[0-9][0-9]?[0-9]?)$").unwrap();

    let Some(caps) = re.captures(cmd) else {
        println!("no match!");
        return 0;
    };

    let num = caps["num"].parse::<i32>().unwrap();
    if &caps["direction"] == "L" {
        return -num
    }
    num
}

fn add_modulo_100(a: i32, b: i32) -> i32 {
    let mut result = a + b;
    result %= 100;
    if result < 0 {
        return result + 100
    }
    if result >= 100 {
        return result - 100
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_nils() {
        let input = vec!{"L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"}.into_iter().map(str::to_owned).collect();
        assert_eq!(count_nils(50, input), 3)
    }

    #[test]
    fn test_cmd_to_int() {
        assert_eq!(parse_cmd("L68"), -68);
        assert_eq!(parse_cmd("R48"), 48);
        assert_eq!(parse_cmd("L1"), -1);
        assert_eq!(parse_cmd("R5"), 5);
    }

    #[test]
    fn test_add() {
        assert_eq!(add_modulo_100(50, -68), 82);
        assert_eq!(add_modulo_100(82, -30), 52);
        assert_eq!(add_modulo_100(52, 48), 0);
        assert_eq!(add_modulo_100(0, -5), 95);
        assert_eq!(add_modulo_100(95, 60), 55);
        assert_eq!(add_modulo_100(55, -55), 0);
        assert_eq!(add_modulo_100(0, -1), 99);
        assert_eq!(add_modulo_100(99, -99), 0);
        assert_eq!(add_modulo_100(0, 14), 14);
        assert_eq!(add_modulo_100(14, -82), 32);
    }
}