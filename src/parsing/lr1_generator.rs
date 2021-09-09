//runtimes for these algorithms (specifically insering a new state into the DFA) could definintely be improved

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

struct ItemSet<'a> {
    //key is next symbol to read for that item
    set: HashSet<Item<'a>>,
}

struct DFAState<'a> {
    id: usize,
    itemset: ItemSet<'a>,
    transitions: HashMap<grammar::Symbol, usize>,
}

struct DFA<'a> {
    states: Vec<DFAState<'a>>,
}

impl ItemSet<'_> {
    pub fn print(&self, cfg: &grammar::CFG) {
        for item in &self.set {
            cfg.print_production(item.production);
            print!(" , {}\n", cfg.symbol_str(&item.lookahead));
        }
    }
}

//resource for algorithm: http://www.orcca.on.ca/~watt/home/courses/2007-08/cs447a/notes/LR1%20Parsing%20Tables%20Example.pdf
pub fn generate<'a>(
    cfg: &'a grammar::CFG,
) -> Result<parse_table::Table<'a>, parse_table::TableErr> {
    let firsts = cfg.generate_firsts();
    let dfa = generate_dfa(cfg, &firsts);
    let mut table = parse_table::Table {
        rows: Vec::with_capacity(dfa.states.len()),
        cfg: cfg,
    };
    for state in dfa.states {
        match generate_table_row(&state) {
            Ok(r) => table.rows.push(r),
            Err(e) => return Result::Err(e),
        };
    }
    Result::Ok(table)
}

fn generate_table_row(state: &DFAState) -> Result<parse_table::TableRow, parse_table::TableErr> {
    let mut cells = HashMap::new();

    for transition in &state.transitions {
        match transition.0 {
            grammar::Symbol::Terminal(_) => {
                //shift/reduce
                //TODO: this
            }
            grammar::Symbol::Nonterminal(_) => {
                //goto
                match cells.insert(*transition.0, parse_table::TableCell::Goto(*transition.1)) {
                    None => (),
                    //goto/goto conflict
                    Some(x) => {
                        return Err(parse_table::TableErr::Conflict(
                            parse_table::TableCell::Goto(*transition.1),
                            x,
                        ))
                    }
                }
            }
            grammar::Symbol::EOF() => (), //make some kind of accept
            _ => panic!("empty transition in generating lr(1) table"),
        }
    }

    Result::Ok(parse_table::TableRow { cells: cells })
}

fn generate_dfa<'a: 'b, 'b>(
    cfg: &'a grammar::CFG,
    firsts: &'a Vec<HashSet<grammar::Symbol>>,
) -> DFA<'a> {
    //set up start state
    let mut start_set = ItemSet {
        set: HashSet::new(),
    };
    //will always be S' -> S <eof>, <eof>
    let start_item = Item {
        reading: 0,
        production: &cfg.productions[0][0],
        lookahead: grammar::Symbol::EOF(),
    };
    start_set.set.insert(start_item);
    start_set = closure(start_set, cfg, &firsts);
    let mut start_state = DFAState {
        id: 0,
        itemset: start_set,
        transitions: HashMap::new(),
    };

    //set up dfa
    let mut dfa = DFA { states: Vec::new() };
    dfa.states.push(start_state);
    //expand dfa
    let mut added = true;
    //should change data structure but idc at the moment. just want it to woooork
    while added {
        //buffers
        let mut to_add = Vec::new();
        let mut transitions = Vec::new();
        added = false;
        for i in 0..dfa.states.len() {
            let state = &dfa.states[i];
            //get all itemsets needed for this state
            let adj_states = get_dfa_tranitions(state, cfg, firsts);
            //find or create id for each itemset
            let mut flag = false;
            for adj in adj_states {
                for j in 0..dfa.states.len() {
                    if adj.1.set == dfa.states[j].itemset.set {
                        transitions.push((i, adj.0, j)); //found id
                        flag = true;
                        break;
                    }
                }
                if !flag {
                    //not found
                    //to_add.push(adj.1);
                    let new_id = dfa.states.len() + transitions.len();
                    to_add.push(DFAState {
                        id: new_id,
                        itemset: adj.1,
                        transitions: HashMap::new(),
                    });
                    transitions.push((i, adj.0, dfa.states.len() + transitions.len()));
                    added = true;
                }
            }
        }
        //populate the real objects from the buffers
        /*for new_state in to_add {
            dfa.states.push(new_state);
        }
          for transition in transitions {
              dfa.states[transition.0]
                  .transitions
                  .insert(transition.1, transition.2);
          }*/
    }
    dfa
}

fn itemsets_equal(a: &ItemSet, b: &ItemSet) -> bool {
    if a.set.len() != b.set.len() {
        return false;
    }
    for a_item in &a.set {
        if !b.set.contains(a_item) {
            return false;
        }
    }
    true
}

fn get_dfa_tranitions<'a: 'b, 'b>(
    state: &'b DFAState<'b>,
    cfg: &'a grammar::CFG,
    firsts: &'a Vec<HashSet<grammar::Symbol>>,
) -> HashMap<grammar::Symbol, ItemSet<'b>> {
    let mut map: HashMap<grammar::Symbol, ItemSet> = HashMap::new();
    for item in &state.itemset.set {
        match map.get_mut(&item.lookahead) {
            Some(itemset) => {
                itemset.set.insert(*item);
            }
            None => {
                let mut set = HashSet::new();
                set.insert(*item);
                map.insert(item.lookahead, ItemSet { set: set });
            }
        };
    }
    let mut res: HashMap<grammar::Symbol, ItemSet> = HashMap::new();
    for item in map {
        res.insert(item.0, closure(item.1, cfg, firsts));
    }

    res
}
//populates an itemset with the closure of its items
fn closure<'a: 'b, 'b>(
    mut itemset: ItemSet<'b>,
    cfg: &'b grammar::CFG,
    firsts: &'a Vec<HashSet<grammar::Symbol>>,
) -> ItemSet<'b> {
    let mut add_buf;
    let mut keep_going = true;
    while keep_going {
        keep_going = false;
        add_buf = HashSet::new(); //may want to change this later
        for item in &itemset.set {
            closure_item(cfg, firsts, &item, &mut add_buf);
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
                    lookahead: x.clone(),
                    production: prod,
                });
            }
        }
        grammar::FirstSet::Nonterminal(x) => {
            for lookahead in x {
                for prod in prods {
                    dest.insert(Item {
                        reading: item.reading,
                        lookahead: lookahead.clone(),
                        production: prod,
                    });
                }
            }
        }
    }
}
