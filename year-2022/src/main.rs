mod day01;
mod day02;
mod day03;
mod day04;

use std::fs;
use std::io;

fn main() {
    let mut question = String::new();
    println!("Enter Question Number:");
    io::stdin()
        .read_line(&mut question)
        .expect("Question number was expected!");
    let path_to_inputs = format!("inputs/input{}.txt", question.trim());
    let contents = String::from(fs::read_to_string(path_to_inputs)
        .expect("Cannot read file!").trim());
    let vec: Vec<&str> = contents
        .split("\n").collect();
    let question_number = String::from(question);
    match question_number.trim() {
        "1" => {
            println!("Day 1 Part 1: {}", day01::solve_1_part_1(vec.clone()));
            println!("Day 1 Part 2: {}", day01::solve_1_part_2(vec.clone()));
        },
        "2" => {
            println!("Day 2 Part 1: {}", day02::solve_2_part_1(vec.clone()));
            println!("Day 2 Part 2: {}", day02::solve_2_part_2(vec.clone()));
        },
        "3" => {
            println!("Day 3 Part 1: {}", day03::solve_3_part_1(vec.clone()));
            println!("Day 3 Part 2: {}", day03::solve_3_part_2(vec.clone()));
        },
        // "4" => {
        //     println!("Day 4 Part 1: {}", day04::solve_4_part_1(vec.clone()));
        //     println!("Day 4 Part 2: {}", day04::solve_4_part_2(vec.clone()));
        // },
        _ => println!("Question not found! {}", question_number.as_str()=="1"),
    };
}
