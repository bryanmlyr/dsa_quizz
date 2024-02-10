use colored::Colorize;
use quizz::{ask_about_space_complexity, ask_about_time_complexity, ask_for_complexity_type, get_random_question, Stats};
use utils::read_file_to_json;

mod quizz;
mod utils;

const QUIZZ_FILE_PATH: &str = "./assets/quizz.json";

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
