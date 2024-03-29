use std::io::stdin;
use colored::Colorize;

const DATA_STRUCTURES_KEY: &str = "data_structures";
const TIME_COMPLEXITY_KEY: &str = "time_complexity";
const SPACE_COMPLEXITY_KEY: &str = "space_complexity";
const QUESTION_NAME_KEY: &str = "name";

pub struct Stats {
    pub correct: u8,
    pub incorrect: u8,
    pub total: u8,
}

pub fn get_random_question(content: &serde_json::Value) -> &serde_json::Value {
    let questions = content[DATA_STRUCTURES_KEY].as_array().unwrap();
    let random_index = rand::random::<usize>() % questions.len();

    &questions[random_index]
}

fn compare_answers(user_answer: &str, correct_answer: &str, stats: &mut Stats) {
    stats.total += 1;
    if user_answer.replace(" ", "").trim().to_lowercase() == correct_answer.replace(" ", "").to_lowercase() {
        println!("{}", "Correct!".green());
        stats.correct += 1;
    } else {
        println!("{} {}", "Incorrect! The correct answer is".red(), correct_answer.red().italic());
        stats.incorrect += 1;
    }
}

pub fn ask_for_complexity_type() -> u8 {
    println!("Choose the complexity type");
    println!("0. Time complexity");
    println!("1. Space complexity");

    let mut complexity_type = String::new();
    stdin().read_line(&mut complexity_type).unwrap();

    complexity_type.trim().parse().unwrap_or_else(|_| {
        println!("Invalid input, please try again");
        ask_for_complexity_type()
    })
}

pub fn ask_about_time_complexity(question: &serde_json::Value, stats: &mut Stats) {
    println!("What is the time complexity of the following function?");
    let time_complexity = question[TIME_COMPLEXITY_KEY].as_object().unwrap();
    let question_name = question[QUESTION_NAME_KEY].as_str().unwrap();

    let random_operation = rand::random::<usize>() % time_complexity.len();
    let picked_key = time_complexity.keys().nth(random_operation).unwrap();
    let picked_value = time_complexity.get(picked_key).unwrap().as_str().unwrap();

    println!("{} - {}: ", question_name, picked_key);
    let mut user_answer = String::new();
    stdin().read_line(&mut user_answer).unwrap();

    compare_answers(&user_answer, picked_value, stats);
}

pub fn ask_about_space_complexity(question: &serde_json::Value, stats: &mut Stats) {
    println!("What is the space complexity of the following function?");
    let space_complexity = question[SPACE_COMPLEXITY_KEY].as_str().unwrap();
    let question_name = question[QUESTION_NAME_KEY].as_str().unwrap();

    println!("{} - {}: ", question_name, space_complexity);
    let mut user_answer = String::new();
    stdin().read_line(&mut user_answer).unwrap();

    compare_answers(&user_answer, space_complexity, stats);
}