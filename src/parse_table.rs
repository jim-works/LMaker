use super::grammar;

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

#[derive(Copy, Clone)]
struct DFAItem<'a> {
    reading: usize,
    production: &'a grammar::CFGProduction,
    next_terminal: i32,
}

fn add_nonterminal<'a: 'b, 'b>(
    cfg: &'a grammar::CFG,
    nonterminal: usize,
    itemset: &mut Vec<DFAItem<'b>>,
) {
    let productions = &cfg.productions[nonterminal];
    for prod in productions {
        let item = DFAItem {
            reading: 0,
            production: prod,
            next_terminal: 0,
        };
        itemset.push(item);
        match prod.rhs[0] {
            grammar::Symbol::Nonterminal(x) if x != nonterminal => add_nonterminal(cfg, x, itemset), //maybe catch circular productions here. or do validation beforehand
            _ => (),
        }
    }
}

fn generate_state(cfg: &grammar::CFG, from: Vec<DFAItem>) {
    let mut itemset: Vec<DFAItem> = Vec::new();
    for item in from {
        itemset.push(item);
        if item.production.rhs.len() <= item.reading {
            continue; //done, no need to create new ones
        }
        match item.production.rhs[item.reading] {
            grammar::Symbol::Nonterminal(x) => {
                let mut tmp = itemset;
                add_nonterminal(cfg, x, &mut tmp);
                itemset = tmp;
            }
            _ => continue,
        }
    }
}

pub fn generate_table_slr(grammar: &grammar::CFG) {
    //generate DFA
}
