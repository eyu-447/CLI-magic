use cli_magic::*;
use std::thread;
use std::time::Duration;

fn main() {
    Logger::info("🚀 Starting CLI Magic demo...");

    // ===== Styled Text Demo =====
    println!("{}", style("Welcome to CLI Magic!").red().bold());
    println!("{}", style("Chainable styles demo").green().bold().italic().underline());

    // ===== Prompt System Demo =====
    let name = Prompt::input("Enter your name");
    let color_choice = Prompt::select(
        "Pick your favorite color",
        &["Red", "Green", "Blue", "Yellow", "Cyan", "Magenta"]
    );
    let secret = Prompt::password("Enter your secret password");

    Logger::success(&format!(
        "Name: {}, Color: {}, Secret length: {}",
        name, color_choice, secret.len()
    ));

    // ===== Spinner Demo =====
    let mut sp = Spinner::new("Loading spinner...");
    sp.start();
    thread::sleep(Duration::from_millis(1500));
    sp.success("Spinner finished successfully!");

    // ===== Progress Bar Demo =====
    let mut pb = ProgressBar::new(20);
    for i in 0..=20 {
        pb.set_position(i);
        thread::sleep(Duration::from_millis(100));
    }
    pb.finish("Progress bar complete!");

    // ===== Table Renderer Demo =====
    let mut table = table::renderer::Table::new(vec!["Name", "Favorite Color", "Secret Length"]);
    table.add_row(vec![&name, &color_choice, &secret.len().to_string()]);
    table.render();

    // ===== Logger Levels Demo =====
    Logger::debug("Debug message example");
    Logger::info("Info message example");
    Logger::warn("Warning message example");
    Logger::error("Error message example");
    Logger::success("Success message example");

    Logger::info("🎉 CLI Magic demo finished!");
}
