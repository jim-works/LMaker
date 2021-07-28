pub enum TableCell {
    Shift(i32),
    Reduce(i32),
    Goto(i32),
}

pub struct TableRow {
    pub cells: Vec<TableCell>,
}

pub struct Table {
    pub rows: Vec<TableRow>,
}
