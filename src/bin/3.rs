use regex::Regex;
use std::fs;

/// extracts all the mul(a,b) where a and b are string for 1 to 3 digits numbers
/// returns a vector of tubles (a,b, index) where a and b are the numbers and index is the index of the match in the input string
fn preprocess(input: String) -> Vec<(usize , usize , usize )> {
    let mul_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    mul_pattern
        .captures_iter(input.as_str())
        .map(|mul_match| {
            (
                {
                    // caution index stats at 1
                    let a = mul_match.get(1).unwrap().as_str();
                    a.parse().unwrap()
                },
                {
                    let b = mul_match.get(2).unwrap().as_str();
                    b.parse().unwrap()
                },
                {
                    // get position of the match in the input string
                    mul_match.get(0).unwrap().start()
                }
            )
        })
        .collect()
}


/// find all of the don't() and do() in the input string
/// returns a vector of tuples (state, index) where state is a bool true if do() and false if don't()
fn find_dont_do(input: String) -> Vec<(bool, usize)> {
    let dont_pattern = Regex::new(r"(don't|do)\(\)").unwrap();
    
    dont_pattern
        .captures_iter(input.as_str())
        .map(|dont_match| {
            (
                {
                    let state = dont_match.get(1).unwrap().as_str();
                    state == "do"
                },
                {
                    // get position of the match in the input string
                    dont_match.get(0).unwrap().start()
                }
            )
        })
        .collect()
}


fn compute_mul_and_sum(muls: Vec<(usize , usize , usize)>) -> usize {
    muls.iter().map(|(a, b, _c)| a*b).sum()
}

fn conditionnal_comput_mul_and_sum(input : String) -> usize {
    let muls = preprocess(input.clone());
    let dont_dos = find_dont_do(input.clone());

    let mut current_state = true; // start with do()
    let mut current_index: usize = 0; // start before the first match
    let mut sum = 0;
    for (state, index) in dont_dos {
        if current_state == false {
            current_state = state;
            current_index = index;
        }
        else {
            muls.iter().filter(|(_a, _b, c)| *c >= current_index && *c <= index).for_each(|(a, b, _c)| {
                
                sum += a*b;
            });
            current_state = state;
            current_index = index;
            
        }
    }
    // last region
    if current_state == true {
        muls.iter().filter(|(_a, _b, c)| *c >= current_index).for_each(|(a, b, _c)| {
            sum += a*b;
        });
    }
    sum
}

fn main() {
    let input = fs::read_to_string("inputs/3.txt").unwrap();
    println!("part 1 : {}", compute_mul_and_sum(preprocess(input.clone())));
    println!("part 2 : {}", conditionnal_comput_mul_and_sum(input));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("inputs/3_e.txt").unwrap();
        
        assert_eq!(compute_mul_and_sum(preprocess(input)), 161);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("inputs/3_e2.txt").unwrap();
        assert_eq!(conditionnal_comput_mul_and_sum(input), 48);
    }
}
