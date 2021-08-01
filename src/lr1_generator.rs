use super::grammar;
use super::parse_table;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Item<'a> {
    reading: usize,
    production: &'a grammar::CFGProduction,
    lookahead: grammar::Symbol,
}

struct ItemSet<'a: 'b, 'b> {
    //key is next symbol to read for that item
    set: HashSet<Item<'b>>,
    cfg: &'a grammar::CFG<'a>,
}

pub fn generate(cfg: &grammar::CFG) /*-> parse_table::Table*/
{
    let firsts = cfg.generate_firsts();
    let mut start_set = ItemSet {
        set: HashSet::new(),
        cfg: &cfg,
    };
    let start_item = Item {
        reading: 0,
        production: &cfg.productions[0][0],
        lookahead: grammar::Symbol::EOF(),
    };
    start_set.set.insert(start_item);
    start_set = closure(start_set, &firsts);
    //other stuff
    for item in start_set.set {
        cfg.print_production(item.production);
        print!(" , {}\n", cfg.symbol_str(&item.lookahead));
    }
}

fn closure<'a: 'b, 'b: 'c, 'c>(
    mut itemset: ItemSet<'b, 'c>,
    firsts: &'a Vec<HashSet<grammar::Symbol>>,
) -> ItemSet<'b, 'c> {
    let mut add_buf;
    let mut keep_going = true;
    while keep_going {
        keep_going = false;
        add_buf = HashSet::new(); //may want to change this later
        for item in &itemset.set {
            closure_item(itemset.cfg, firsts, &item, &mut add_buf);
        }
        for add in add_buf.into_iter() {
            if itemset.set.insert(add) {
                keep_going = true;
            }
        }
    }
    itemset
}

//adds items to dest
fn closure_item<'a: 'b, 'b: 'c, 'c>(
    cfg: &'a grammar::CFG,
    firsts: &'b Vec<HashSet<grammar::Symbol>>,
    item: &'c Item,
    dest: &'c mut HashSet<Item<'b>>,
) {
    let nt;
    match item.production.rhs[item.reading] {
        grammar::Symbol::Nonterminal(x) => nt = x,
        _ => return,
    }
    let prods = &cfg.productions[nt];
    let beta = &item.production.rhs[item.reading + 1..item.production.rhs.len()];
    let lookahead_option = cfg.get_first(beta, firsts);
    let lookaheads;
    match lookahead_option {
        Some(x) => lookaheads = x,
        None => lookaheads = grammar::FirstSet::Other(item.lookahead),
    }
    match lookaheads {
        grammar::FirstSet::Other(x) => {
            for prod in prods {
                dest.insert(Item {
                    reading: item.reading,
                    lookahead: x,
                    production: prod,
                });
            }
        }
        grammar::FirstSet::Nonterminal(x) => {
            for lookahead in x {
                for prod in prods {
                    dest.insert(Item {
                        reading: item.reading,
                        lookahead: *lookahead,
                        production: prod,
                    });
                }
            }
        }
    }
}
