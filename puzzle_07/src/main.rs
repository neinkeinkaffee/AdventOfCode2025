use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    println!("Welcome to Puzzle 07!");

    let input = read_to_string("input.txt").unwrap();
    let splits = count_splits(&input);
    println!("Number of beam splits: {splits}");

    let timelines = count_timelines(&input);
    println!("Number of alternate timelines: {timelines}");
}

fn count_splits(input: &str) -> u32 {
    let mut lines = input.split("\n").filter(|l| !l.is_empty());

    let start_index = lines.next().unwrap().find("S").unwrap();
    let mut beams = HashSet::from([start_index]);
    let mut split_count = 0;

    for (i, row) in lines.enumerate() {
        if i%2 == 0 {
            continue
        }
        let mut new_beams = HashSet::new();
        for beam_index in &beams {
            if row.chars().nth(*beam_index).unwrap() == '^' {
                split_count += 1;
                new_beams.insert(*beam_index-1);
                new_beams.insert(*beam_index+1);
            } else {
                new_beams.insert(*beam_index);
            }
        }
        beams = new_beams;
    }

    split_count
}

fn count_timelines(input: &str) -> u128 {
    let mut lines = input.split("\n").filter(|l| !l.is_empty());

    let mut timelines: HashMap<usize, u128> = HashMap::new();
    let start_index = lines.next().unwrap().find("S").unwrap();
    timelines.insert(start_index, 1);

    for (i, row) in lines.enumerate() {
        if i % 2 == 0 { continue }
        let mut new_timelines: HashMap<usize, u128> = HashMap::new();
        for (parent_index, parent_timeline_count) in timelines {
            if row.chars().nth(parent_index).unwrap() == '^' {
                let left_timeline_count = new_timelines.entry(parent_index-1).or_insert(0);
                *left_timeline_count += parent_timeline_count;
                let right_timeline_count = new_timelines.entry(parent_index+1).or_insert(0);
                *right_timeline_count += parent_timeline_count;
            } else {
                let middle_timeline_count = new_timelines.entry(parent_index).or_insert(0);
                *middle_timeline_count += parent_timeline_count;
            }
        }
        timelines = new_timelines;
    }

    timelines.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_splits() {
        let input = ".......S.......\n\
                           ...............\n\
                           .......^.......\n\
                           ...............\n\
                           ......^.^......\n\
                           ...............\n\
                           .....^.^.^.....\n\
                           ...............\n\
                           ....^.^...^....\n\
                           ...............\n\
                           ...^.^...^.^...\n\
                           ...............\n\
                           ..^...^.....^..\n\
                           ...............\n\
                           .^.^.^.^.^...^.\n\
                           ...............\n";

        assert_eq!(count_splits(input), 21);
    }

    #[test]
    fn test_count_timelines() {
        let input = ".......S.......\n\
                           ...............\n\
                           .......^.......\n\
                           ...............\n\
                           ......^.^......\n\
                           ...............\n\
                           .....^.^.^.....\n\
                           ...............\n\
                           ....^.^...^....\n\
                           ...............\n\
                           ...^.^...^.^...\n\
                           ...............\n\
                           ..^...^.....^..\n\
                           ...............\n\
                           .^.^.^.^.^...^.\n\
                           ...............\n";

        assert_eq!(count_timelines(input), 40);
    }
}