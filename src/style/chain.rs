use std::fmt;

pub struct StyledText {
    text: String,
    style_sequence: Vec<String>,
}

impl StyledText {
    pub fn new(text: &str) -> Self {
        StyledText {
            text: text.to_string(),
            style_sequence: vec![],
        }
    }

    pub fn red(mut self) -> Self { self.style_sequence.push("\x1b[31m".to_string()); self }
    pub fn green(mut self) -> Self { self.style_sequence.push("\x1b[32m".to_string()); self }
    pub fn blue(mut self) -> Self { self.style_sequence.push("\x1b[34m".to_string()); self }
    pub fn yellow(mut self) -> Self { self.style_sequence.push("\x1b[33m".to_string()); self }
    pub fn magenta(mut self) -> Self { self.style_sequence.push("\x1b[35m".to_string()); self }
    pub fn cyan(mut self) -> Self { self.style_sequence.push("\x1b[36m".to_string()); self }

    pub fn light_red(mut self) -> Self { self.style_sequence.push("\x1b[91m".to_string()); self }
    pub fn light_green(mut self) -> Self { self.style_sequence.push("\x1b[92m".to_string()); self }
    pub fn light_yellow(mut self) -> Self { self.style_sequence.push("\x1b[93m".to_string()); self }
    pub fn light_blue(mut self) -> Self { self.style_sequence.push("\x1b[94m".to_string()); self }
    pub fn light_magenta(mut self) -> Self { self.style_sequence.push("\x1b[95m".to_string()); self }
    pub fn light_cyan(mut self) -> Self { self.style_sequence.push("\x1b[96m".to_string()); self }

    pub fn dark_red(mut self) -> Self { self.style_sequence.push("\x1b[31;1m".to_string()); self }
    pub fn dark_green(mut self) -> Self { self.style_sequence.push("\x1b[32;1m".to_string()); self }
    pub fn dark_yellow(mut self) -> Self { self.style_sequence.push("\x1b[33;1m".to_string()); self }
    pub fn dark_blue(mut self) -> Self { self.style_sequence.push("\x1b[34;1m".to_string()); self }
    pub fn dark_magenta(mut self) -> Self { self.style_sequence.push("\x1b[35;1m".to_string()); self }
    pub fn dark_cyan(mut self) -> Self { self.style_sequence.push("\x1b[36;1m".to_string()); self }

    pub fn bold(mut self) -> Self { self.style_sequence.push("\x1b[1m".to_string()); self }
    pub fn underline(mut self) -> Self { self.style_sequence.push("\x1b[4m".to_string()); self }
    pub fn italic(mut self) -> Self { self.style_sequence.push("\x1b[3m".to_string()); self }
    pub fn blink(mut self) -> Self { self.style_sequence.push("\x1b[5m".to_string()); self }
}


impl fmt::Display for StyledText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for code in &self.style_sequence {
            result.push_str(code);
        }
        result.push_str(&self.text);
        result.push_str("\x1b[0m"); // reset
        write!(f, "{}", result)
    }
}

// دالة مساعدة للبدء بالـ chain
pub fn style(text: &str) -> StyledText {
    StyledText::new(text)
}
