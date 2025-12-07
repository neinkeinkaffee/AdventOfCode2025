use std::fs::read_to_string;

fn main() {
    println!("Welcome to Puzzle 03!");

    let mut sum = 0;
    let lines: Vec<String> = read_to_string("input.txt").unwrap().lines().map(String::from).collect();

    for line in lines {
        sum += max_joltage(line)
    }

    println!("Total joltage: {sum}")
}

fn max_joltage(battery: String) -> u128 {
    let joltages: Vec<u128> = battery.chars()
        .map(|c| { u128::from(c.to_digit(10).unwrap()) })
        .collect();
    let mut switched_on = vec![false; joltages.len()];

    let (m1_index, m1) = left_most_max(0, joltages.len(), &joltages, &mut switched_on);

    if m1_index == joltages.len()-1 {
        let (_, m0) = left_most_max(0, m1_index, &joltages, &mut switched_on);
        return m0 * 10 + m1
    }
    let (_, m2) = right_most_max(m1_index+1, joltages.len(), &joltages, &mut switched_on);

    m1 * 10 + m2
}

fn left_most_max(start: usize, end: usize, elements: &[u128], skip: &mut Vec<bool>) -> (usize, u128) {
    let mut max = elements[start];
    let mut max_index = start;

    for i in start..end {
        let x = elements[i];
        if max < x  && !skip[i] {
            max = x;
            max_index = i;
        }
    }
    skip[max_index] = true;

    (max_index, max)
}

fn right_most_max(start: usize, end: usize, elements: &[u128], skip: &mut Vec<bool>) -> (usize, u128) {
    let mut max = elements[start];
    let mut max_index = start;

    for i in start..end {
        let x = elements[i];
        if max <= x && !skip[i] {
            max = x;
            max_index = i;
        }
    }
    skip[max_index] = true;

    (max_index, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_most_max() {
        assert_eq!(right_most_max(0, 4, &vec!{1, 2, 3, 1}, &mut vec![false; 4]), (2, 3));
        assert_eq!(right_most_max(0, 4, &vec!{1, 2, 3, 3}, &mut vec![false; 4]), (3, 3));
        assert_eq!(right_most_max(0, 4, &vec!{1, 2, 3, 3}, &mut vec! {false, false, false, true}), (2, 3));
    }

    #[test]
    fn test_left_most_max() {
        assert_eq!(left_most_max(0, 4, &vec!{1, 2, 3, 1}, &mut vec![false; 4]), (2, 3));
        assert_eq!(left_most_max(0, 4, &vec!{1, 2, 3, 3}, &mut vec![false; 4]), (2, 3));
        assert_eq!(left_most_max(0, 4, &vec!{1, 2, 3, 3}, &mut vec! {false, false, true, false}), (3, 3));
    }

    #[test]
    fn test_max_joltage() {
        assert_eq!(max_joltage("987654321111111".to_string()), 98);
        assert_eq!(max_joltage("811111111111119".to_string()), 89);
        assert_eq!(max_joltage("234234234234278".to_string()), 78);
        assert_eq!(max_joltage("818181911112111".to_string()), 92);
    }

    #[test]
    fn test_max_joltage_12() {
        assert_eq!(max_joltage("987654321111111".to_string()), 987654321111);
        assert_eq!(max_joltage("811111111111119".to_string()), 811111111119);
        assert_eq!(max_joltage("234234234234278".to_string()), 434234234278);
        assert_eq!(max_joltage("818181911112111".to_string()), 888911112111);
    }
}