use cli_magic::*;

fn main() {
    Logger::info("Starting CLI Magic demo...");

    let name = Prompt::input("Enter your name");
    println!("Hello {}", style(&name).green().bold());

    let choice = Prompt::select("Choose your favorite fruit", &["Apple", "Banana", "Cherry"]);
    Logger::success(&format!("You chose {}", choice));

    let mut spinner = Spinner::new("Loading...");
    spinner.start();
    std::thread::sleep(std::time::Duration::from_secs(2));
    spinner.success("Done!");

    let mut pb = ProgressBar::new(100);
    for i in 0..=100 {
        pb.set_position(i);
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    pb.finish("Progress complete!");

    let mut table = Table::new(vec!["Name", "Fruit"]);
    table.add_row(vec![&name, &choice]);
    table.render();
}
