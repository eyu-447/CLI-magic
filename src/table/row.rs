use crate::table::cell::Cell;

pub struct Row {
    pub cells: Vec<Cell>,
}

impl Row {
    pub fn new(cells: Vec<Cell>) -> Self {
        Row { cells }
    }
}
