use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum Symbol {
    Terminal(usize),
    Nonterminal(usize),
    Empty(),
    EOF(),
}

pub enum FirstSet<'a> {
    Other(Symbol),
    Nonterminal(&'a [Symbol]),
}

pub struct CFGProduction {
    pub nonterminal: usize,
    pub rhs: Vec<Symbol>,
}

pub struct CFG<'a> {
    //first vec: lhs id, next vec: list of productions where that nonterminal is the lhs
    pub productions: Vec<Vec<CFGProduction>>,
    pub nonterminal_symbols: Vec<&'a str>,
    pub terminal_symbols: Vec<&'a str>,
}

fn set_has_nonterm(sets: &[HashMap<Symbol, Symbol>]) -> bool {
    for set in sets {
        for symbol in set.keys() {
            match symbol {
                Symbol::Nonterminal(_) => return true,
                _ => (),
            };
        }
    }
    false
}

impl CFG<'_> {
    //generate the first set for each nonterminal in a CFG.
    //example: if A-> b|c|De and D->d|<empty>, first(A)={b,c,d,e}
    //return- first vec is indexed by nonterminal id, second vec contains list of terminals in the first set
    pub fn generate_firsts(&self) -> Vec<Vec<Symbol>> {
        let mut firsts = Vec::new();
        //prepopulate list
        for nt in 0..self.nonterminal_symbols.len() {
            firsts = self.generate_first_from_prods(nt, firsts);
        }
        //we do passes until all nonterminals are eliminated from the first sets
        let mut iter = 0;
        loop {
            iter += 1;
            for nonterminal in 0..self.nonterminal_symbols.len() {
                let (to_add, to_remove) = self.generate_first_nonterm(nonterminal, &mut firsts);
                for item in to_remove {
                    firsts[nonterminal].remove(&item);
                }
                for item in to_add {
                    firsts[nonterminal].insert(item, item);
                }
            }
            if !set_has_nonterm(&firsts) || iter > 100 {
                println!("broken iter={}", iter);
                break;
            }
        }
        //transform firsts into desired structure
        let mut ret = Vec::new();
        for first in firsts {
            let first_list = first.keys().map(|&x| x).collect();
            ret.push(first_list);
        }
        ret
    }
    //generates the first set for a specific nonterminal from it's productions
    //this is the set of possible first terminals for a derviation of the nonterminal
    //this function is only one step of generate_firsts, it will not remove nonterminals in the resulting set
    fn generate_first_from_prods(
        &self,
        nonterminal: usize,
        mut firsts: Vec<HashMap<Symbol, Symbol>>,
    ) -> Vec<HashMap<Symbol, Symbol>> {
        let mut my_firsts = HashMap::new();
        for prod in &self.productions[nonterminal] {
            for symbol in &prod.rhs {
                match symbol {
                    Symbol::Empty() => {
                        my_firsts.insert(*symbol, *symbol);
                    }
                    _ => {
                        my_firsts.insert(*symbol, *symbol);
                        break;
                    }
                }
            }
        }
        firsts.push(my_firsts);
        firsts
    }
    //loops through each inprogress first set and replaces nonterminals with their first sets
    fn generate_first_nonterm(
        &self,
        nonterminal: usize,
        firsts: &Vec<HashMap<Symbol, Symbol>>,
    ) -> (Vec<Symbol>, Vec<Symbol>) {
        let mut to_add = Vec::new();
        let mut to_remove = Vec::new();
        for symbol in firsts[nonterminal].keys() {
            match symbol {
                Symbol::Nonterminal(x) => {
                    to_remove.push(*symbol);
                    if *x != nonterminal {
                        //don't want to add my first set to itself
                        for other_item in firsts[*x].keys() {
                            match other_item {
                                Symbol::Empty() => {
                                    //if the other nonterminal conatins the empty string, we need to add our own first set to it
                                    for my_item in firsts[nonterminal].keys() {
                                        to_add.push(*my_item);
                                    }
                                }
                                _ => (),
                            }
                            to_add.push(*other_item);
                        }
                    }
                }
                _ => (),
            }
        }
        return (to_add, to_remove);
    }

    //generates the first set for a string of symbols
    //this is the set of possible first terminals in the derived string
    //example "aBc" -> {a}, "<empty>aBc" -> {a}, "Bc" -> {a,b,c} if B -> a|b|<empty>, "<empty><empty><empty>" -> None
    pub fn get_first<'a>(
        &self,
        string: &[Symbol],
        firsts: &'a [Vec<Symbol>],
    ) -> Option<FirstSet<'a>> {
        match string.first() {
            Some(symbol) => match symbol {
                Symbol::Terminal(_) | Symbol::EOF() => Option::Some(FirstSet::Other(*symbol)),
                Symbol::Nonterminal(x) => Option::Some(FirstSet::Nonterminal(&firsts[*x])),
                Symbol::Empty() => self.get_first(&string[1..], firsts),
            },
            None => Option::None,
        }
    }

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
    //start rule is S'-> S where S is the first nonterminal
    pub fn from_strings(strings: &[String]) -> CFG {
        struct RHS<'a> {
            prod: CFGProduction,
            iter: std::str::SplitWhitespace<'a>,
        }
        let mut productions: Vec<Vec<CFGProduction>> = Vec::new();
        productions.push(Vec::new()); //for the created start rule
        let mut nt_map: HashMap<&str, usize> = HashMap::new(); //0 is reserved for created start rule: S' -> S$
        let mut nt_symbols: Vec<&str> = Vec::new();
        //set up start rule
        nt_symbols.push("S'");
        productions[0].push(CFGProduction {
            nonterminal: 0,
            rhs: vec![Symbol::Nonterminal(1), Symbol::EOF()],
        });
        let mut nt_last: usize = 0;
        let mut t_map: HashMap<&str, usize> = HashMap::new();
        let mut t_symbols: Vec<&str> = Vec::new();
        let mut t_last: usize = 0;
        let mut rh_sides: Vec<RHS> = Vec::new();
        for string in strings {
            let mut iter = string.split_whitespace();
            //LHS
            let lhs;
            match iter.next() {
                Some(s) => {
                    lhs = CFG::register(s, &mut nt_last, &mut nt_map);
                    if lhs >= productions.len() {
                        productions.push(Vec::new());
                        nt_symbols.push(s);
                    }
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
                    Some('.') if elem.len() == 1 => Symbol::Empty(),
                    Some('.') => {
                        let n = CFG::register(
                            &elem[1..elem.len()], //remove . from start of terminal
                            &mut t_last,
                            &mut t_map,
                        );
                        if n == t_last {
                            //register in terminal symbol table
                            t_symbols.push(&elem[1..elem.len()]);
                        }
                        Symbol::Terminal(n - 1) //n is one higher than the corresponding string
                    }
                    None => panic!("No char!"), //empty somehow
                    _ => Symbol::Nonterminal(CFG::register(elem, &mut nt_last, &mut nt_map)), //nonterminal
                };
                rhs.prod.rhs.push(symbol);
            }
            //combine into ruleset
            productions[rhs.prod.nonterminal].push(rhs.prod);
        }
        return CFG {
            productions: productions,
            nonterminal_symbols: nt_symbols,
            terminal_symbols: t_symbols,
        };
    }
    //pretty prints to stdout
    pub fn print(&self) {
        for lhs_symbol in self.productions.iter() {
            for production in lhs_symbol.iter() {
                print!("{} ->", self.nonterminal_symbols[production.nonterminal]);
                for symbol in production.rhs.iter() {
                    match symbol {
                        Symbol::Terminal(x) => print!(" .{}", self.terminal_symbols[*x]),
                        Symbol::Nonterminal(x) => print!(" {}", self.nonterminal_symbols[*x]),
                        Symbol::Empty() => print!(" .<empty>"),
                        Symbol::EOF() => print!(" .<eof>"),
                    }
                }
                println!();
            }
        }
    }
}
