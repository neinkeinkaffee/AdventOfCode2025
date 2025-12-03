use std::fs::read_to_string;

fn main() {
    println!("Welcome to Puzzle 02!");

    let lines = read_to_string("input.txt").unwrap();

    let ranges = parse_input(lines);

    let mut sum = 0;
    for (start, end) in ranges {
        let invalid_numbers = find_invalid(start, end);
        for invalid_number in invalid_numbers {
            sum += invalid_number;
        }
    }

    println!("Sum of invalid numbers: {sum}");
}

fn parse_input(input: String) -> Vec<(i128, i128)> {
    let mut ranges: Vec<(i128, i128)> = vec![];
    let raw_ranges = input.split(",");
    for raw_range in raw_ranges {
        let split: Vec<&str> = raw_range.split("-").collect();
        if split.len() == 2 {
            let raw_start = split.get(0).unwrap();
            let raw_end = split.get(1).unwrap();
            // println!("Split: {raw_start}-{raw_end}");
            let start = raw_start.parse::<i128>().unwrap();
            let end = raw_end.parse::<i128>().unwrap();
            ranges.push((start, end));
        }
    }
    ranges
}

fn find_invalid(start: i128, end: i128) -> Vec<i128> {
    let mut invalid_numbers: Vec<i128> = vec![];
    for n in start..=end {
        if is_invalid(n) {
            invalid_numbers.push(n);
        }
    }
    invalid_numbers
}

fn is_invalid(n: i128) -> bool {
    let s: String = n.to_string();
    if s.len() % 2 == 1 {
        return false;
    }
    let l = s.len() / 2;
    let repeat = &s[0..l];
    // println!("testing '{repeat}'");
    let mut allmatch = true;
    for k in 1..(s.len() / l) {
        let left = k * l;
        let right = k * l + l;
        // println!("  checking {left}..{right}");
        let candidate = &s[left..right];
        if candidate != repeat {
            // println!("    {candidate} != {repeat}");
            allmatch = false;
        }
        if !allmatch {
            break;
        }
    }
    if allmatch {
        // println!("We haven't returned, {n} must be invalid");
        return true;
    }
    // println!("NOT all match!");
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("1-2,34-781,222-910".to_string()),
            vec! {(1, 2), (34, 781), (222, 910)}
        );
    }

    #[test]
    fn test_is_invalid() {
        assert_eq!(is_invalid(1000), false);
        assert_eq!(is_invalid(11), true);
        assert_eq!(is_invalid(10), false);
        assert_eq!(is_invalid(101), false);
        assert_eq!(is_invalid(111), false);
        assert_eq!(is_invalid(1111), true);
        assert_eq!(is_invalid(111111), true);
        assert_eq!(is_invalid(22), true);
        assert_eq!(is_invalid(99), true);
        assert_eq!(is_invalid(1010), true);
        assert_eq!(is_invalid(1188511885), true);
        assert_eq!(is_invalid(222222), true);
        assert_eq!(is_invalid(446446), true);
        assert_eq!(is_invalid(38593859), true);
    }

    #[test]
    fn test_find_invalid() {
        assert_eq!(find_invalid(11, 22), vec! {11, 22});
        assert_eq!(find_invalid(95, 115), vec! {99});
        assert_eq!(find_invalid(998, 1012), vec! {1010});
        assert_eq!(find_invalid(1188511880, 1188511890), vec! {1188511885});
        assert_eq!(find_invalid(222220, 222224), vec! {222222});
        assert_eq!(find_invalid(1698522, 1698528), vec! {});
        assert_eq!(find_invalid(446443, 446449), vec! {446446});
        assert_eq!(find_invalid(38593856, 38593862), vec! {38593859});
        assert_eq!(find_invalid(565653, 565659), vec! {});
        assert_eq!(find_invalid(824824821, 824824827), vec! {});
        assert_eq!(find_invalid(2121212118, 2121212124), vec! {});
    }
}
