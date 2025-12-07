use std::fs::read_to_string;

fn main() {
    println!("Welcome to Puzzle 03!");

    let mut sum = 0;
    let lines: Vec<String> = read_to_string("input.txt").unwrap().lines().map(String::from).collect();

    for line in lines {
        sum += max_joltage(line, 12)
    }

    println!("Total joltage: {sum}")
}

fn left_most_max_index(start: usize, end: usize, elements: &[u128], skip: &mut Vec<bool>) -> Option<usize> {
    let mut max = 0;
    let mut max_index = None;

    for i in start..end {
        let x = elements[i];
        if max < x && !skip[i] {
            max = x;
            max_index = Some(i);
        }
    }

    match max_index {
        Some(i) => {
            skip[i] = true;
            Some(i)
        }
        None => {
            None
        }
    }
}

fn right_most_max_index(start: usize, end: usize, elements: &[u128], skip: &mut Vec<bool>) -> Option<usize> {
    let mut max = 0;
    let mut max_index = None;

    for i in start..end {
        let x = elements[i];
        if max <= x && !skip[i] {
            max = x;
            max_index = Some(i);
        }
    }

    match max_index {
        Some(i) => {
            skip[i] = true;
            Some(i)
        }
        None => {
            None
        }
    }
}

fn max_joltage(battery: String, num_batteries: i32) -> u128 {
    let joltages: Vec<u128> = battery.chars()
        .map(|c| { u128::from(c.to_digit(10).unwrap()) })
        .collect();
    let mut switched_on = vec![false; joltages.len()];
    let mut battery_count = 0;

    let mut l: Vec<usize> = vec!{0};
    let mut r = joltages.len();
    let mut m = left_most_max_index(*l.last().unwrap(), r, &joltages, &mut switched_on).unwrap();
    l.push(m);
    battery_count += 1;

    while battery_count < num_batteries {
         match left_most_max_index(m+1, r, &joltages, &mut switched_on) {
             Some(_m) => {
                 m = _m;
                 l.push(m+1);
                 battery_count += 1;
             }
             None => {
                 r = m;
                 match left_most_max_index(*l.last().unwrap(), r, &joltages, &mut switched_on) {
                     Some(_m) => {
                         m = _m;
                         l.push(m+1);
                         battery_count += 1;
                     }
                     None => {
                         _ = l.pop().unwrap()
                     }
                 }
             }
        }
    }

    let mut total = 0;
    for i in 0..joltages.len() {
        if switched_on[i] {
            total = total*10 + joltages[i]
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_joltage() {
        assert_eq!(max_joltage("818181911112111".to_string(), 2), 92);
        assert_eq!(max_joltage("818181911112111".to_string(), 3), 921);
        assert_eq!(max_joltage("818181911112111".to_string(), 4), 9211);
        assert_eq!(max_joltage("818181911112111".to_string(), 5), 92111);
        assert_eq!(max_joltage("818181911112111".to_string(), 6), 912111);
        assert_eq!(max_joltage("818181911112111".to_string(), 7), 9112111);
        assert_eq!(max_joltage("818181911112111".to_string(), 9), 911112111);
        assert_eq!(max_joltage("818181911112111".to_string(), 10), 8911112111);
        assert_eq!(max_joltage("818181911112111".to_string(), 11), 88911112111);
        assert_eq!(max_joltage("818181911112111".to_string(), 12), 888911112111);

        assert_eq!(max_joltage("234234234234278".to_string(), 2), 78);
        assert_eq!(max_joltage("234234234234278".to_string(), 3), 478);
        assert_eq!(max_joltage("234234234234278".to_string(), 4), 4478);
        assert_eq!(max_joltage("234234234234278".to_string(), 5), 44478);
        assert_eq!(max_joltage("234234234234278".to_string(), 6), 444478);
        assert_eq!(max_joltage("234234234234278".to_string(), 7), 4444278);
        assert_eq!(max_joltage("234234234234278".to_string(), 8), 44434278);
        assert_eq!(max_joltage("234234234234278".to_string(), 9), 444234278);
        assert_eq!(max_joltage("234234234234278".to_string(), 10), 4434234278);
        assert_eq!(max_joltage("234234234234278".to_string(), 11), 44234234278);
        assert_eq!(max_joltage("234234234234278".to_string(), 12), 434234234278);

        assert_eq!(max_joltage("987654321111111".to_string(), 2), 98);
        assert_eq!(max_joltage("811111111111119".to_string(), 2), 89);
        assert_eq!(max_joltage("234234234234278".to_string(), 2), 78);
        assert_eq!(max_joltage("818181911112111".to_string(), 2), 92);

        assert_eq!(max_joltage("987654321111111".to_string(), 12), 987654321111);
        assert_eq!(max_joltage("811111111111119".to_string(), 12), 811111111119);
        assert_eq!(max_joltage("234234234234278".to_string(), 12), 434234234278);
        assert_eq!(max_joltage("818181911112111".to_string(), 12), 888911112111);
    }

    #[test]
    fn test_right_most_max() {
        assert_eq!(right_most_max_index(0, 4, &vec!{1, 2, 3, 4}, &mut vec![true; 4]), None);
        assert_eq!(right_most_max_index(0, 4, &vec!{1, 1, 1, 1}, &mut vec![false; 4]), Some(3));
        assert_eq!(right_most_max_index(0, 4, &vec!{1, 2, 2, 1}, &mut vec![false; 4]), Some(2));
        assert_eq!(right_most_max_index(0, 4, &vec!{1, 1, 1, 1}, &mut vec!{false, false, false, true}), Some(2));
    }

    #[test]
    fn test_left_most_max() {
        assert_eq!(left_most_max_index(0, 4, &vec!{1, 2, 3, 4}, &mut vec![true; 4]), None);
        assert_eq!(left_most_max_index(0, 4, &vec!{1, 1, 1, 1}, &mut vec![false; 4]), Some(0));
        assert_eq!(left_most_max_index(0, 4, &vec!{1, 2, 2, 1}, &mut vec![false; 4]), Some(1));
        assert_eq!(left_most_max_index(0, 4, &vec!{1, 1, 1, 1}, &mut vec! {true, false, false, false}), Some(1));
    }
}