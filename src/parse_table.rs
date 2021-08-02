use super::grammar::*;
use std::collections::HashMap;

pub enum TableCell {
    Shift(usize),
    Reduce(usize),
    Goto(usize),
    Accept(),
}

pub enum TableErr {
    Conflict(TableCell, TableCell),
}

pub struct TableRow {
    pub cells: HashMap<Symbol, TableCell>,
}

pub struct Table<'a> {
    pub rows: Vec<TableRow>,
    pub cfg: &'a CFG<'a>,
}
