pub struct Theme {
    pub name: String,
    pub foreground: String,
    pub background: String,
}

pub fn light_theme() -> Theme {
    Theme {
        name: "light".to_string(),
        foreground: "\x1b[37m".to_string(),
        background: "\x1b[40m".to_string(),
    }
}

pub fn dark_theme() -> Theme {
    Theme {
        name: "dark".to_string(),
        foreground: "\x1b[37m".to_string(),
        background: "\x1b[40m".to_string(),
    }
}
