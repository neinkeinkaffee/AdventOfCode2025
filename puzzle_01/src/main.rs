use regex::Regex;
use std::fs::read_to_string;

fn main() {
    println!("Welcome to Puzzle 01!");

    let start_state = 50;

    let lines: Vec<String> = read_to_string("input.txt").unwrap().lines().map(String::from).collect();
    let len_input = lines.len();
    println!("Input length: {len_input}");
    let nil_count = count_nils(start_state, lines);
    println!("Nil count: {nil_count}");

    let lines = read_to_string("input.txt").unwrap().lines().map(String::from).collect();
    let more_nils_count = count_more_nils(start_state, lines);
    println!("More nils count: {more_nils_count}");
}

fn count_nils(start_state: i32, lines: Vec<String>) -> i32 {
    let mut nil_count = 0;
    let mut state = start_state;
    for line in lines {
        state = sum_mod_100(state, parse_cmd(line.trim()));
        if state == 0 {
            nil_count += 1
        }
    }
    nil_count
}

fn count_more_nils(start_state: i32, lines: Vec<String>) -> i32 {
    let mut nil_count = 0;
    let mut state = start_state;
    for line in lines {
        let cmd = parse_cmd(line.trim());
        let (next_state, nils) = sum_mod_and_div_100(state, cmd);
        state = next_state;
        nil_count += nils;
    }
    nil_count
}

fn parse_cmd(cmd: &str) -> i32 {
    let re = Regex::new(r"^(?<direction>[R|L])(?<num>[0-9][0-9]?[0-9]?[0-9]?)$").unwrap();

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

fn sum_mod_100(a: i32, b: i32) -> i32 {
    (a + b).rem_euclid(100)
}

fn sum_mod_and_div_100(a: i32, b: i32) -> (i32, i32) {
    if a == -(b%100) {
        return (0, -(b/100)+1)
    }
    let mut nils = ((a + b) / 100).abs();
    let rem_euclidian = (a + b).rem_euclid(100);
    let rem_non_euclidian = (a + b) % 100;
    if rem_euclidian != rem_non_euclidian && a != 0 {
        nils += 1;
    }
    println!("input: {a}, {b}");
    println!("output: {rem_euclidian}, {nils}");
    (rem_euclidian, nils)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_more_nils() {
        assert_eq!(count_more_nils(50, vec!{"R50"}.into_iter().map(str::to_owned).collect()), 1);
        assert_eq!(count_more_nils(50, vec!{"R150"}.into_iter().map(str::to_owned).collect()), 2);
        assert_eq!(count_more_nils(50, vec!{"L150"}.into_iter().map(str::to_owned).collect()), 2);
        assert_eq!(count_more_nils(50, vec!{"R1000"}.into_iter().map(str::to_owned).collect()), 10);
        assert_eq!(count_more_nils(50, vec!{"L1000"}.into_iter().map(str::to_owned).collect()), 10);
        assert_eq!(count_more_nils(50, vec!{"L68"}.into_iter().map(str::to_owned).collect()), 1);
        assert_eq!(count_more_nils(50, vec!{"L68", "L30", "R48"}.into_iter().map(str::to_owned).collect()), 2);
        assert_eq!(count_more_nils(50, vec!{"L68", "L30", "R48", "L5", "R60"}.into_iter().map(str::to_owned).collect()), 3);
        assert_eq!(count_more_nils(50, vec!{"L68", "L30", "R48", "L5", "R60", "L55"}.into_iter().map(str::to_owned).collect()), 4);
        assert_eq!(count_more_nils(50, vec!{"L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99"}.into_iter().map(str::to_owned).collect()), 5);
        assert_eq!(count_more_nils(50, vec!{"L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"}.into_iter().map(str::to_owned).collect()), 6);
    }

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
    fn test_sum_mod_100() {
        assert_eq!(sum_mod_100(50, -68), 82);
        assert_eq!(sum_mod_100(82, -30), 52);
        assert_eq!(sum_mod_100(52, 48), 0);
        assert_eq!(sum_mod_100(0, -5), 95);
        assert_eq!(sum_mod_100(95, 60), 55);
        assert_eq!(sum_mod_100(55, -55), 0);
        assert_eq!(sum_mod_100(0, -1), 99);
        assert_eq!(sum_mod_100(99, -99), 0);
        assert_eq!(sum_mod_100(0, 14), 14);
        assert_eq!(sum_mod_100(14, -82), 32);
    }

    #[test]
    fn test_sum_mod_and_div_100() {
        assert_eq!(sum_mod_and_div_100(50, -68), (82, 1));
        assert_eq!(sum_mod_and_div_100(82, -30), (52, 0));
        assert_eq!(sum_mod_and_div_100(52, 48), (0, 1));
        assert_eq!(sum_mod_and_div_100(0, -5), (95, 0));
        assert_eq!(sum_mod_and_div_100(95, 60), (55, 1));
        assert_eq!(sum_mod_and_div_100(55, -55), (0, 1));
        assert_eq!(sum_mod_and_div_100(0, -1), (99, 0));
        assert_eq!(sum_mod_and_div_100(99, -99), (0, 1));
        assert_eq!(sum_mod_and_div_100(0, 14), (14, 0));
        assert_eq!(sum_mod_and_div_100(14, -82), (32, 1));
        assert_eq!(sum_mod_and_div_100(13, -210), (3, 2));
        assert_eq!(sum_mod_and_div_100(0, -689), (11, 6));
        assert_eq!(sum_mod_and_div_100(50, 1000), (50, 10));
        assert_eq!(sum_mod_and_div_100(50, 1001), (51, 10));
        assert_eq!(sum_mod_and_div_100(50, 150), (0, 2));
        assert_eq!(sum_mod_and_div_100(50, -150), (0, 2));
    }
}