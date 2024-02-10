use std::fs::read_to_string;
use std::io::stdin;
use colored::Colorize;

const QUIZZ_FILE_PATH: &str = "./assets/quizz.json";
const DATA_STRUCTURES_KEY: &str = "data_structures";

struct Stats {
    correct: u8,
    incorrect: u8,
    total: u8,
}

fn read_file(file_path: String) -> Result<String, std::io::Error> {
    read_to_string(file_path)
}

fn read_file_to_json(file_path: String) -> Result<serde_json::Value, serde_json::Error> {
    let content = read_file(file_path);
    match content {
        Ok(content) => serde_json::from_str(&content),
        Err(error) => Err(serde_json::Error::io(error)),
    }
}

fn get_random_question(content: &serde_json::Value) -> &serde_json::Value {
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

fn ask_for_complexity_type() -> u8 {
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

fn ask_about_time_complexity(question: &serde_json::Value, stats: &mut Stats) {
    println!("What is the time complexity of the following function?");
    let time_complexity = question["time_complexity"].as_object().unwrap();
    let question_name = question["name"].as_str().unwrap();

    let random_operation = rand::random::<usize>() % time_complexity.len();
    let picked_key = time_complexity.keys().nth(random_operation).unwrap();
    let picked_value = time_complexity.get(picked_key).unwrap().as_str().unwrap();

    println!("{} - {}: ", question_name, picked_key);
    let mut user_answer = String::new();
    stdin().read_line(&mut user_answer).unwrap();

    compare_answers(&user_answer, picked_value, stats);
}

fn ask_about_space_complexity(question: &serde_json::Value, stats: &mut Stats) {
    println!("What is the space complexity of the following function?");
    let space_complexity = question["space_complexity"].as_str().unwrap();
    let question_name = question["name"].as_str().unwrap();

    println!("{} - {}: ", question_name, space_complexity);
    let mut user_answer = String::new();
    stdin().read_line(&mut user_answer).unwrap();

    compare_answers(&user_answer, space_complexity, stats);
}

fn main() {
    let complexity_type: u8 = ask_for_complexity_type();
    let content = read_file_to_json(QUIZZ_FILE_PATH.to_string()).unwrap();
    let mut current_stats = Stats {
        correct: 0,
        incorrect: 0,
        total: 0,
    };

    loop {
        let question = get_random_question(&content);
        match complexity_type {
            0 => ask_about_time_complexity(question, &mut current_stats),
            1 => ask_about_space_complexity(question, &mut current_stats),
            _ => println!("Invalid complexity type"),
        }

        println!("{}", "_".repeat(80).blue().bold());
        println!("Correct: {}, Incorrect: {}, Total: {}",
                 current_stats.correct.to_string().green(),
                 current_stats.incorrect.to_string().red(),
                 current_stats.total.to_string().blue().bold()
        );
        println!("{}", "_".repeat(80).blue().bold());
    }
}
