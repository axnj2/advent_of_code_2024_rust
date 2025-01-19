use std::fs;

fn main() {
    let final_result : i32;
    let exemple_input = fs::read_to_string("inputs/1_e.txt")
                                    .expect("The file should have opened");
    final_result = part1(exemple_input);
    

    println!("the final result is : {}", final_result);
}

fn part1(input : String) -> i32 {
    let split_string = input.lines()
                            .map(|line| line.trim().split(" "));
                        
}


