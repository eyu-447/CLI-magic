use std::io::{self, Write};

pub struct Prompt;

impl Prompt {
    pub fn input(message: &str) -> String {
        print!("{}: ", message);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    pub fn password(message: &str) -> String {
        rpassword::prompt_password(&format!("{}: ", message)).unwrap()
    }

    pub fn confirm(message: &str) -> bool {
        loop {
            let ans = Prompt::input(&format!("{} (y/n)", message));
            match ans.to_lowercase().as_str() {
                "y" | "yes" => return true,
                "n" | "no" => return false,
                _ => println!("Please enter y or n"),
            }
        }
    }

    pub fn select(message: &str, options: &[&str]) -> String {
        loop {
            println!("{}", message);
            for (i, option) in options.iter().enumerate() {
                println!("{}: {}", i + 1, option);
            }
            let choice = Prompt::input("Enter choice number");
            if let Ok(index) = choice.parse::<usize>() {
                if index > 0 && index <= options.len() {
                    return options[index - 1].to_string();
                }
            }
            println!("Invalid choice, try again.");
        }
    }
}
