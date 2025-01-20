use itertools::Itertools;
use std::fs;

// learned from https://github.com/voberle/adventofcode/blob/main/2024/day01/src/main.rs
// when trying to bench mark the difference,
// I found virtually no execution time difference at both 15 ms

fn main() {
    let exemple_input = fs::read_to_string("inputs/1_e.txt").expect("The file should have opened");
    let exemple_lists = extract_number_lists(exemple_input);

    println!("exemple : {}", part1(&exemple_lists.0, &exemple_lists.1));

    let full_input = fs::read_to_string("inputs/1.txt").unwrap();
    let full_lists = extract_number_lists(full_input);

    println!("step 1 : {}", part1(&full_lists.0, &full_lists.1));

    println!(
        "step 2 exemple : {}",
        part2(&exemple_lists.0, &exemple_lists.1)
    );

    println!("step 2 : {}", part2(&full_lists.0, &full_lists.1));
}

fn extract_number_lists(input: String) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split_ascii_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .into_iter()
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}

fn part1(first_list: &Vec<i32>, second_list: &Vec<i32>) -> u32 {
    first_list
        .iter()
        .sorted_unstable()
        .zip(second_list.iter().sorted_unstable())
        .map(|(e1, e2)| e1.abs_diff(*e2))
        .sum()
}

fn part2(first_list: &Vec<i32>, second_list: &Vec<i32>) -> i32 {
    first_list
        .iter()
        .map(|e1| second_list.iter().filter(|e2| *e2 == e1).sum::<i32>())
        .sum()
}
