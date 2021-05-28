use std::collections::HashMap;

pub enum Symbol {
    Terminal(usize),
    Nonterminal(usize),
}

pub struct CFGProduction {
    pub nonterminal: usize,
    pub rhs: Vec<Symbol>,
}

pub struct CFG {
    pub productions: Vec<Vec<CFGProduction>>,
}

//TODO make this work at all lol
impl CFG {
    //start state is first item in strings
    //terminals start with dot
    //nonterminals are anything not starting with dot
    //whitespace separating each element
    //each entry in strings is a production: term -> .number .* term
    fn register<'a>(k: &'a str, last: &mut usize, m: &mut HashMap<&'a str, usize>) -> usize {
        match m.get(&k) {
            Some(n) => *n,
            None => {
                *last += 1;
                let v = *last;
                m.insert(k, v);
                v
            }
        }
    }
    //assuming valid grammar with all nonterminals having at least 1 production
    pub fn from_strings(strings: &Vec<String>) -> CFG {
        struct RHS<'a> {
            prod: CFGProduction,
            iter: std::str::SplitWhitespace<'a>,
        }
        let mut productions: Vec<Vec<CFGProduction>> = Vec::new();
        productions.push(Vec::new()); //for the created start rule
        let mut nt_map: HashMap<&str, usize> = HashMap::new(); //0 is reserved for created start rule: S' -> S$
        let mut nt_last: usize = 0;
        let mut t_map: HashMap<&str, usize> = HashMap::new();
        let mut t_last: usize = 0;
        let mut rh_sides: Vec<RHS> = Vec::new();
        for string in strings {
            let mut iter = string.split_whitespace();
            //LHS
            let mut lhs = 0;
            match iter.next() {
                Some(s) => {
                    lhs = CFG::register(s, &mut nt_last, &mut nt_map);
                    if lhs > productions.len() {
                        productions.push(Vec::new());
                    }
                    println!("{}", lhs)
                }
                None => panic!("No LHS!"),
            }
            let prod = CFGProduction {
                nonterminal: lhs,
                rhs: Vec::new(),
            };
            //arrow
            match iter.next() {
                Some("->") => {}
                _ => panic!("Expected ->"),
            };
            rh_sides.push(RHS {
                prod: prod,
                iter: iter,
            });
        }
        //RHS
        for mut rhs in rh_sides {
            for elem in rhs.iter {
                let symbol: Symbol = match elem.chars().next() {
                    Some('.') => Symbol::Terminal(CFG::register(
                        &elem[1..elem.len()], //remove . from start of terminal
                        &mut t_last,
                        &mut t_map,
                    )),
                    None => panic!("No char!"), //empty somehow
                    _ => Symbol::Nonterminal(CFG::register(elem, &mut nt_last, &mut nt_map)), //nonterminal
                };
                rhs.prod.rhs.push(symbol);
            }
            //combine into ruleset
        }
        return CFG {
            productions: productions,
        };
    }
}
