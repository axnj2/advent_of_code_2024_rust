use regex::Regex;
use std::fs;

/// extracts all the mul(a,b) where a and b are string for 1 to 3 digits numbers
fn preprocess(input: String) -> Vec<(i32, i32)> {
    let mul_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    
    mul_pattern
        .captures_iter(input.as_str())
        .map(|mul_match| {
            (
                {
                    let a =  mul_match.get(0).unwrap().as_str();
                    print!("{}",a);
                    a.parse().unwrap()

                    
                },
                mul_match.get(1).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect()
}

fn compute_mul_and_sum(muls: Vec<(i32, i32)>) -> i32 {
    println!("{:?}",muls);

    muls.iter().map(|(a, b)| a*b).sum()
}

fn main() {
    let input = fs::read_to_string("inputs/3.txt").unwrap();
    println!("part 1 : {}", compute_mul_and_sum(preprocess(input)))
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

    //     #[test]
    //     fn test_part2() {
    //         let input = fs::read_to_string("inputs/3_e.txt").unwrap();
    //         assert_eq!(count_safe_reports_dampened(&preprocess(&input)), 4);
    //     }
}
