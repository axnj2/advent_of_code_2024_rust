use std::collections::HashSet;
use std::fs;
use std::iter::Iterator;

fn preprocess(input: &String) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn count_safe_reports(reports: &Vec<Vec<u32>>) -> u32 {
    // let intermediary_result: Vec<_> = reports
    //     .iter()
    //     .filter(|report| {
    //         (report.is_sorted() || report.iter().rev().is_sorted()) && {
    //             let mut uniq = std::collections::HashSet::new();
    //             // HashSet().insert(e) return true if it's a
    //             // new element and false if it has been seen before
    //             report.iter().all(|&e| uniq.insert(e))
    //         }
    //     })
    //     .collect();
    // ---------------
    // intersting to note that we can use "{:?}" to print most data structures
    // ---------------
    // println!("{:?}", intermediary_result);

    reports
        .iter()
        .filter(|report| is_safe(report))
        .count()
        .try_into()
        .unwrap()
}

fn is_safe(report: &Vec<u32>) -> bool {
    (report.is_sorted() || report.iter().rev().is_sorted())
        && {
            // inforces differs by at least 1
            let mut uniq = HashSet::new();
            // HashSet().insert(e) return true if it's a
            // new element and false if it has been seen before
            report.iter().all(|&e| uniq.insert(e))
        }
        && {
            // https://stackoverflow.com/questions/73832075/how-to-compare-the-value-of-next-element-with-the-current-element-in-an-iterator
            // for the .windows
            report
                .windows(2)
                .all(|element_pair| element_pair[0].abs_diff(element_pair[1]) < 4)
        }
}

fn count_safe_reports_dampened(reports: &Vec<Vec<u32>>) -> u32 {
    reports
        .iter()
        .filter(|report| {
            is_safe(report) || {
                // check with dampening
                (0..report.len()).any(|index: usize| {
                    let mut new_report = report.to_vec();
                    new_report.remove(index.try_into().unwrap());
                    is_safe(&new_report)
                })
            }
        })
        .count()
        .try_into()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("inputs/2.txt").unwrap();
    println!("part 1 : {}", count_safe_reports(&preprocess(&input)));
    println!(
        "part 2 : {}",
        count_safe_reports_dampened(&preprocess(&input))
    ); // not the correct output
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("inputs/2_e.txt").unwrap();
        assert_eq!(count_safe_reports(&preprocess(&input)), 2);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("inputs/2_e.txt").unwrap();
        assert_eq!(count_safe_reports_dampened(&preprocess(&input)), 4);
    }
}
