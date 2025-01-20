use std::fs;
use std::iter;

fn main() {
    let exemple_input = fs::read_to_string("inputs/1_e.txt").expect("The file should have opened");
    let exemple_lists = extract_number_lists(exemple_input);

    println!(
        "exemple : {}",
        part1(exemple_lists.0.clone(), exemple_lists.1.clone())
    );

    let full_input = fs::read_to_string("inputs/1.txt").unwrap();
    let full_lists = extract_number_lists(full_input);

    println!(
        "step 1 : {}",
        part1(full_lists.0.clone(), full_lists.1.clone())
    );

    println!(
        "step 2 exemple : {}",
        part2(exemple_lists.0, exemple_lists.1)
    );

    print!(
        "step 2 : {}",
        part2(full_lists.0, full_lists.1)
    );
}

fn extract_number_lists(input: String) -> (Vec<i32>, Vec<i32>) {
    let split_string = input.lines().map(|line| line.trim().split(" "));
    let first_list: Vec<i32> = split_string
        .clone()
        .map(|mut elements| elements.next().unwrap())
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    let second_list: Vec<i32> = split_string
        .clone()
        .map(|elements| elements.last().unwrap())
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    return (first_list, second_list);
}

fn part1(mut first_list: Vec<i32>, mut second_list: Vec<i32>) -> i32 {
    first_list.sort();
    second_list.sort();

    let mut final_result = 0;

    for items in iter::zip(first_list, second_list) {
        final_result += (items.0 - items.1).abs()
    }

    return final_result;
}

fn part2(first_list: Vec<i32>, second_list: Vec<i32>) -> i32 {
    let mut final_value = 0;
    for value1 in first_list {
        final_value += second_list
            .clone()
            .into_iter()
            .filter(|value2| *value2 == value1)
            .sum::<i32>()
    }

    return final_value;
}
