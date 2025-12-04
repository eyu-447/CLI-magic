pub struct Cell {
    pub content: String,
    pub color: Option<String>,
    pub bold: bool,
}

impl Cell {
    pub fn new(content: &str) -> Self {
        Cell {
            content: content.to_string(),
            color: None,
            bold: false,
        }
    }
}
