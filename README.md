# FREE PALESTINE 🇵🇸

# CLI Magic ✨
**Powerful Rust CLI framework** for building colorful, interactive, and professional command-line applications with ease.  

---

## About the Library 📚
`cli_magic` is a Rust library designed to simplify and speed up the creation of CLI applications.  
It provides a complete toolkit for developers, including:

- **Colorful text styling** (foreground, background, light & dark shades)  
- **Chainable text effects**: bold, underline, italic, blink  
- **Interactive prompts**: text input, password masking, yes/no, select menus  
- **Spinners** with multiple styles and multi-threaded support ⏳  
- **Progress bars**: static or animated, download/upload, percentage + ETA  
- **Logger**: debug, info, warn, error, success with timestamps 📝  
- **Tables**: fully formatted, auto-sized, aligned, and colored 📊  

`cli_magic` is perfect for anyone who wants to create **professional CLI tools** without rewriting common functionalities from scratch.  

---

## Features 🌈
- **Chainable styling**: `style("Hello").red().bold().underline()`  
- **All natural colors** with light and dark variations  
- **Themes support** for consistent CLI appearance  
- **Multi-threaded spinners and animated progress bars**  
- **Flexible table renderer** with smart alignment and color support  
- **Easy-to-use interactive prompts** with validation  

---

## Installation 🚀

### Using GitHub
Add this to your `Cargo.toml`:

```toml
[dependencies]
cli_magic = { git = "https://github.com/YOURUSERNAME/cli-magic.git" }
```


---

## Usage 💻

```rust
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
```

- Chainable text styling ✅  
- Logger with colored levels ✅  
- Interactive prompts ✅  
- Tables and progress bars ready-to-use ✅  

---

## Why CLI Magic? 🌟
- **Save time**: No need to implement common CLI features manually  
- **Professional look**: Colors, spinners, tables, and logs out-of-the-box  
- **Highly customizable**: Add your own colors, themes, or effects  
- **Modular**: Use only the parts you need or the full framework  

---

**License:** MIT  
**Repository:** [https://github.com/YOURUSERNAME/cli-magic](https://github.com/YOURUSERNAME/cli-magic)
