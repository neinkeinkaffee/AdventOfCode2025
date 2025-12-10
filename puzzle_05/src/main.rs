use std::fs::read_to_string;

fn main() {
    println!("Welcome to Puzzle 05!");

    let input: String = read_to_string("input.txt").unwrap();

    let (ranges, ids) = parse_input(input);
    let fresh_ids = count_fresh(ranges, ids);

    println!("Fresh ingredients: {fresh_ids}");
}

fn parse_input(input: String) -> (Vec<(u128, u128)>, Vec<u128>) {
    let lines: Vec<String> = input.lines()
        .map(String::from)
        .collect();
    let mut ranges: Vec<(u128, u128)> = vec!{};
    let mut ids: Vec<u128> = vec!{};
    let mut line_number = 0;
    let mut line;
    while line_number < lines.len() {
        line = &lines[line_number];
        line_number += 1;
        if line.trim().is_empty() {
            break
        }

        let split: Vec<&str> = line.trim().split("-").collect();
        if split.len() == 2 {
            let start = split.get(0).unwrap().parse::<u128>().unwrap();
            let end = split.get(1).unwrap().parse::<u128>().unwrap();
            ranges.push((start, end));
        }
    }

    while line_number < lines.len() {
        line = &lines[line_number];
        line_number += 1;
        if line.trim().is_empty() {
            break
        }
        let id = line.trim().parse::<u128>().unwrap();
        ids.push(id);
    }

    (ranges, ids)
}

fn is_fresh(fresh_ranges: &Vec<(u128, u128)>, id: u128) -> bool {
    for (start, end) in fresh_ranges {
        if *start <= id && id <= *end {
            return true
        }
    }
    false
}

fn count_fresh(fresh_ranges: Vec<(u128, u128)>, ids: Vec<u128>) -> u128 {
    let mut total = 0;
    for id in ids {
        if is_fresh(&fresh_ranges, id) {
            total += 1;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "3-5\n\
                           10-14\n\
                           16-20\n\
                           12-18\n\
                           \n\
                           1\n\
                           5\n\
                           8\n\
                           11\n\
                           17\n\
                           32\n";
        let want_ranges = vec!{(3, 5), (10, 14), (16, 20), (12, 18)};
        let want_ids = vec!{1, 5, 8, 11, 17, 32};
        assert_eq!(parse_input(input.to_string()), (want_ranges, want_ids))
    }

    #[test]
    fn test_is_fresh() {
        assert_eq!(is_fresh(&vec!{(3, 5), (10, 14), (16, 20), (12, 18)}, 1), false);
        assert_eq!(is_fresh(&vec!{(3, 5), (10, 14), (16, 20), (12, 18)}, 5), true);
        assert_eq!(is_fresh(&vec!{(3, 5), (10, 14), (16, 20), (12, 18)}, 8), false);
        assert_eq!(is_fresh(&vec!{(3, 5), (10, 14), (16, 20), (12, 18)}, 11), true);
        assert_eq!(is_fresh(&vec!{(3, 5), (10, 14), (16, 20), (12, 18)}, 17), true);
        assert_eq!(is_fresh(&vec!{(3, 5), (10, 14), (16, 20), (12, 18)}, 32), false);
    }

    #[test]
    fn test_count_fresh() {
        assert_eq!(count_fresh(vec!{(3, 5), (10, 14), (16, 20), (12, 18)}, vec!{1, 5, 8, 11, 17, 32}), 3)
    }
}