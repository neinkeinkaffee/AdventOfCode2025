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
    let (m1_index, m1) = left_most_max(&joltages);

    if m1_index == joltages.len()-1 {
        let (_, m0) = left_most_max(&joltages[0..m1_index]);
        return m0 * 10 + m1
    }

    let (_, m2) = right_most_max(&joltages[m1_index+1..]);
    m1 * 10 + m2
}

fn left_most_max(elements: &[u128]) -> (usize, u128) {
    let mut max = elements[0];
    let mut max_index = 0;

    for (i, x) in elements.iter().enumerate() {
        if max < *x  {
            max = *x;
            max_index = i;
        }
    }

    return (max_index, max);
}

fn right_most_max(elements: &[u128]) -> (usize, u128) {
    let mut max = elements[0];
    let mut max_index = 0;

    for (i, x) in elements.iter().enumerate() {
        if max <= *x  {
            max = *x;
            max_index = i;
        }
    }

    return (max_index, max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        assert_eq!(right_most_max(&vec!{1, 2, 3, 4}), (3, 4))

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