pub fn select_option(message: &str, options: &[&str]) -> String {
    loop {
        println!("{}", message);
        for (i, opt) in options.iter().enumerate() {
            println!("{}: {}", i + 1, opt);
        }
        let choice = crate::prompt::input::input("Enter choice number");
        if let Ok(idx) = choice.parse::<usize>() {
            if idx > 0 && idx <= options.len() {
                return options[idx - 1].to_string();
            }
        }
        println!("Invalid choice, try again.");
    }
}
