use std::fs;

fn parse(raw_input: &String) -> Vec<Vec<i32>> {
    raw_input
        .lines()
        .into_iter()
        .map(|line| {
            line.chars().map(|x| match x {
                'X' => 0,
                'M' => 1,
                'A' => 2,
                'S' => 3,
                _ => -1,
            }).collect()
        })
        .collect()
}

fn count_XMAS_start() {}

fn solve1(input: &String) -> i32 {
    let parsed_input = parse(input);

    todo!()
}

fn main() {
    let input = fs::read_to_string("inputs/4_e.txt").unwrap();
    println!("Part 1 : {}", solve1(&input))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("inputs/4_e.txt").unwrap();

        assert_eq!(solve1(&input), 18);
    }

    // #[test]
    // fn test_part2() {
    //     let input = fs::read_to_string("inputs/3_e2.txt").unwrap();
    //     assert_eq!(conditionnal_comput_mul_and_sum(input), 48);
    // }
}
