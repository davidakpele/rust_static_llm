mod static_qa;

use std::io::{self, Write};
use std::{thread::sleep, time::Duration};

fn main() {
    loop {
        println!("\n=== Terminal Copilot Assistant ===");
        println!("1. Ask a question");
        println!("2. Exit");

        print!("Enter choice (1 or 2): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1" => ask_loop(), // 🔄 Stay in ask mode until user types "exit"
            "2" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option. Please enter 1 or 2."),
        }
    }
}

fn ask_loop() {
    loop {
        print!("\nType your question (or type 'exit' to logout): ");
        io::stdout().flush().unwrap();

        let mut user_question = String::new();
        io::stdin().read_line(&mut user_question).unwrap();

        let trimmed = user_question.trim();

        if trimmed.eq_ignore_ascii_case("exit") {
            println!("👋 Logged out and returned to main menu.");
            break;
        }

        let answers = static_qa::find_answer(trimmed);

        if answers.is_empty() {
            println!("\n❌ I can't respond to this question.\n");
        } else {
            for (i, answer) in answers.iter().enumerate() {
                if i == 0 {
                    println!("\n💡 Answer:");
                } else {
                    println!("\n🔀 OR try this:");
                }
                type_out(answer);
                println!();
            }
        }
        
    }
}

fn type_out(text: &str) {
    for ch in text.chars() {
        print!("{}", ch);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(30));
    }
}
