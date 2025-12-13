use std::fs::read_to_string;

fn main() {
    println!("Welcome to Puzzle 05!");

    let input: String = read_to_string("input.txt").unwrap();

    let (ranges, ids) = parse_input(input);
    let fresh_ids = count_stocked_ids_in_range(ranges.clone(), ids);
    let total_ids = count_total_ids_in_range(ranges);

    println!("Fresh ingredients in stock: {fresh_ids}");
    println!("Total number of fresh ingredients: {total_ids}");
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

fn in_range(fresh_ranges: &Vec<(u128, u128)>, id: u128) -> bool {
    for (start, end) in fresh_ranges {
        if *start <= id && id <= *end {
            return true
        }
    }
    false
}

fn count_stocked_ids_in_range(ranges: Vec<(u128, u128)>, filter_ids: Vec<u128>) -> u128 {
    let mut total = 0;

    for id in filter_ids {
        if in_range(&ranges, id) {
            total += 1;
        }
    }

    total
}

fn count_total_ids_in_range(fresh_ranges: Vec<(u128, u128)>) -> u128 {
    let merged_ranges = merge_ranges(fresh_ranges);

    let mut total = 0;
    for (start, end) in merged_ranges {
        let inc = (end - start) + 1;
        total += inc;
    }

    total
}

fn merge_ranges(ranges: Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    let mut merged_ranges = vec!{};

    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut ranges_iter = ranges.iter();
    if let Some(next) = ranges_iter.next() {
        merged_ranges.push(*next)
    } else {
        return merged_ranges;
    }

    while let Some(next) = ranges_iter.next() {
        let this = merged_ranges.pop().unwrap();
        if next.0 <= this.1 {
            if this.1 <= next.1 {
                merged_ranges.push((this.0, next.1))
            } else {
                merged_ranges.push((this.0, this.1))
            }
        } else {
            merged_ranges.push(this);
            merged_ranges.push(*next)
        }
    }

    merged_ranges
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
    fn test_in_range() {
        assert_eq!(in_range(&vec!{(3, 5), (10, 14), (16, 20), (12, 18)}, 1), false);
        assert_eq!(in_range(&vec!{(3, 5), (10, 14), (16, 20), (12, 18)}, 5), true);
        assert_eq!(in_range(&vec!{(3, 5), (10, 14), (16, 20), (12, 18)}, 8), false);
        assert_eq!(in_range(&vec!{(3, 5), (10, 14), (16, 20), (12, 18)}, 11), true);
        assert_eq!(in_range(&vec!{(3, 5), (10, 14), (16, 20), (12, 18)}, 17), true);
        assert_eq!(in_range(&vec!{(3, 5), (10, 14), (16, 20), (12, 18)}, 32), false);
    }

    #[test]
    fn test_count_in_range_filtered() {
        assert_eq!(count_stocked_ids_in_range(vec!{(3, 5), (10, 14), (16, 20), (12, 18)}, vec!{1, 5, 8, 11, 17, 32}), 3)
    }

    #[test]
    fn test_count_in_range() {
        assert_eq!(count_total_ids_in_range(vec!{(3, 5)}), 3);
        assert_eq!(count_total_ids_in_range(vec!{(3, 5), (10, 14)}), 8);
        assert_eq!(count_total_ids_in_range(vec!{(3, 5), (10, 14), (16, 20)}), 13);
        assert_eq!(count_total_ids_in_range(vec!{(3, 5), (10, 14), (12, 18), (16, 20)}), 14);
    }

    #[test]
    fn test_merge_ranges() {
        assert_eq!(merge_ranges(vec!{(3, 5), (10, 14)}), vec!{(3, 5), (10, 14)});
        assert_eq!(merge_ranges(vec!{(3, 5), (10, 14), (12, 18), (16, 20)}), vec!{(3, 5), (10, 20)});
        assert_eq!(merge_ranges(vec!{(3, 5), (10, 14), (12, 13), (16, 20)}), vec!{(3, 5), (10, 14), (16, 20)});
    }
}