use std::fs;

fn parse(raw_input: &String) -> Vec<Vec<i32>> {
    raw_input
        .lines()
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    'X' => 1,
                    'M' => 2,
                    'A' => 3,
                    'S' => 4,
                    _ => -1,
                })
                .collect()
        })
        .collect()
}

fn count_XMAS_start(window: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    let correct_xmas = vec![1, 2, 3, 4];
    // XMAS on the left
    // line index 3 element index 3 to 6 (included)
    count += (0..4).all(|ii| window[3][ii + 3] == correct_xmas[ii]) as i32;
    // XMAS on the right
    // line index 3 element index 3 to 0
    count += (0..4).all(|ii| window[3][3 - ii] == correct_xmas[ii]) as i32;
    // XMAS on the top
    // column index 3 line index 3 to 0
    count += (0..4).all(|ii| window[3 - ii][3] == correct_xmas[ii]) as i32;
    // XMAS on the bottom
    // column index 3 line index 3 to 6
    count += (0..4).all(|ii| window[3 + ii][3] == correct_xmas[ii]) as i32;
    // XMAS diag NW
    // line index 3 to 0 column index 3 to 0
    count += (0..4).all(|ii| window[3 - ii][3 - ii] == correct_xmas[ii]) as i32;
    // XMAS diag NE
    // line index 3 to 0 column index 3 to 6
    count += (0..4).all(|ii| window[3 - ii][3 + ii] == correct_xmas[ii]) as i32;
    // XMAS diag SW
    // line index 3 to 6 column index 3 to 0
    count += (0..4).all(|ii| window[3 + ii][3 - ii] == correct_xmas[ii]) as i32;
    // XMAS diag SE
    // line index 3 to 6 column index 3 to 6
    count += (0..4).all(|ii| window[3 + ii][3 + ii] == correct_xmas[ii]) as i32;

    count
}

fn solve1(input: &String) -> i32 {
    let parsed_input = parse(input);

    let mut count = 0;
    for ii in 0..parsed_input.len() as i32 {
        for jj in 0..parsed_input[0].len() as i32 {
            let mut window: Vec<Vec<i32>> = vec![vec![-1; 7]; 7];
            for i in -3..4 {
                for j in -3..4 {
                    if i + ii >= 0
                        && i + ii < parsed_input.len() as i32
                        && j + jj >= 0
                        && j + jj < parsed_input[0].len() as i32
                    {
                        window[(i + 3) as usize][(j + 3) as usize] =
                            parsed_input[(jj + j) as usize][(ii + i) as usize]
                    }
                }
            }
            count += count_XMAS_start(window)
        }
    }

    count
}

fn count_XMAS_start_2(window: Vec<Vec<i32>>) -> i32 {
    (vec![
        window[3][3] == 3,
        window[2][2] == 2,
        window[4][2] == 2,
        window[4][4] == 4,
        window[2][4] == 4,
    ]
    .iter()
    .all(|x| *x) 
    ||
    vec![
        window[3][3] == 3,
        window[2][2] == 4,
        window[4][2] == 4,
        window[4][4] == 2,
        window[2][4] == 2,
    ]
    .iter()
    .all(|x| *x)
    ||
    vec![
        window[3][3] == 3,
        window[2][2] == 4,
        window[4][2] == 2,
        window[4][4] == 2,
        window[2][4] == 4,
    ]
    .iter()
    .all(|x| *x)
    ||
    vec![
        window[3][3] == 3,
        window[2][2] == 2,
        window[4][2] == 4,
        window[4][4] == 4,
        window[2][4] == 2,
    ]
    .iter()
    .all(|x| *x)) as i32
}

fn solve2(input: &String) -> i32 {
    let parsed_input = parse(input);

    let mut count = 0;
    for ii in 0..parsed_input.len() as i32 {
        for jj in 0..parsed_input[0].len() as i32 {
            let mut window: Vec<Vec<i32>> = vec![vec![-1; 7]; 7];
            for i in -3..4 {
                for j in -3..4 {
                    if i + ii >= 0
                        && i + ii < parsed_input.len() as i32
                        && j + jj >= 0
                        && j + jj < parsed_input[0].len() as i32
                    {
                        window[(i + 3) as usize][(j + 3) as usize] =
                            parsed_input[(jj + j) as usize][(ii + i) as usize]
                    }
                }
            }
            count += count_XMAS_start_2(window)
        }
    }

    count
}

fn main() {
    let input = fs::read_to_string("inputs/4.txt").unwrap();
    println!("Part 1 : {}", solve1(&input));
    println!("Part 2 : {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use std::vec;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("inputs/4_e.txt").unwrap();

        assert_eq!(solve1(&input), 18);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("inputs/4_e.txt").unwrap();

        assert_eq!(solve2(&input), 9);
    }

    #[test]
    fn test_count_XMAS_start() {
        let mut test_window: Vec<Vec<i32>> = vec![vec![-1; 7]; 7];
        test_window[3][3] = 1;
        test_window[3][4] = 2;
        test_window[3][5] = 3;
        test_window[3][6] = 4;

        assert_eq!(count_XMAS_start(test_window.clone()), 1);
        test_window[3][3] = 1;
        test_window[4][4] = 2;
        test_window[5][5] = 3;
        test_window[6][6] = 4;
        assert_eq!(count_XMAS_start(test_window.clone()), 2);
    }
}
