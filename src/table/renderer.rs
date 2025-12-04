pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub column_widths: Vec<usize>,
}

impl Table {
    pub fn new(headers: Vec<&str>) -> Self {
        let widths = headers.iter().map(|h| h.len()).collect();
        Table {
            headers: headers.iter().map(|h| h.to_string()).collect(),
            rows: vec![],
            column_widths: widths,
        }
    }

    pub fn add_row(&mut self, row: Vec<&str>) {
        for (i, cell) in row.iter().enumerate() {
            if cell.len() > self.column_widths[i] {
                self.column_widths[i] = cell.len();
            }
        }
        self.rows.push(row.iter().map(|c| c.to_string()).collect());
    }

    pub fn render(&self) {
        let mut line = String::new();
        for (i, header) in self.headers.iter().enumerate() {
            line.push_str(&format!(" {:width$} |", header, width = self.column_widths[i]));
        }
        println!("{}", line);

        let separator: String = self.column_widths.iter().map(|w| "-".repeat(*w + 2) + "+").collect();
        println!("{}", separator);

        for row in &self.rows {
            let mut line = String::new();
            for (i, cell) in row.iter().enumerate() {
                line.push_str(&format!(" {:width$} |", cell, width = self.column_widths[i]));
            }
            println!("{}", line);
        }
    }
}
