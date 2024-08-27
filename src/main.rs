use reqwest::blocking::get;
use std::io::{self, Write};
use serde_json::Value;
use rand::prelude::SliceRandom;

fn main() {
    println!("What category? ");
    io::stdout().flush().expect("Failed to flush stdout");

    let cats = get("https://opentdb.com/api_category.php")
        .expect("Failed to send request")
        .text()
        .expect("Failed to get response");
    let cats: Value = serde_json::from_str(&cats).expect("Failed to parse categories");

    for (mut i, cat) in cats["trivia_categories"].as_array().expect("Failed to get categories").iter().enumerate() {
        i += 1;
        println!("{i}. {}", cat["name"].as_str().expect("Failed to get category name"));
    }

    let mut category = String::new();
    io::stdin().read_line(&mut category).expect("Failed to read line");
    let category_index = category.trim().parse::<usize>().expect("Failed to parse category index");
    let category_id = cats["trivia_categories"][category_index-1]["id"].as_i64().expect("Failed to get category id");

    print!("Difficulty (easy/medium/hard): ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut diff = String::new();
    io::stdin().read_line(&mut diff).expect("Failed to read difficulty");
    let diff = diff.trim();

    let questions = get(&format!(
        "https://opentdb.com/api.php?amount=1&category={}&difficulty={}&type=multiple",
        category_id, diff
    ))
    .expect("Failed to get question")
    .text()
    .expect("Failed to get question response");
    let _q: Value = serde_json::from_str(&questions).expect("Failed to parse question");
    let question = _q["results"][0].clone();

    let name = &cats["trivia_categories"][category_index-1]["name"].as_str().expect("Failed to get category name").to_lowercase();
    let diff = diff.to_lowercase();

    println!("Selected {diff} {name}");

    println!(
        "{}",
        question["question"]
            .as_str()
            .expect("Failed to get question as string")
    );
    let mut answers = vec![
        question["correct_answer"]
            .as_str()
            .expect("Failed to get correct answer"),
    ];
    answers.extend(
        question["incorrect_answers"]
            .as_array()
            .expect("Failed to get incorrect answers")
            .iter()
            .map(|a| a.as_str().expect("Failed to get incorrect answer")),
    );
    answers.shuffle(&mut rand::thread_rng());

    for (mut i, answer) in answers.iter().enumerate() {
        i += 1;
        println!("{i}. {answer}");
    }

    loop {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read line");
    let a_index: usize = a.trim().parse::<usize>().expect("Failed to parse category index");
    let a = answers[a_index-1];
    if a == "give_up" {
        println!("The answer was {}", question["correct_answer"]);
        break;
    } else if a == question["correct_answer"] {
        println!("Correct!");
        break;
    }else{
        println!("Incorrect.");
    }
    }
}