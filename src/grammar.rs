use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum Symbol {
    Terminal(usize),
    Nonterminal(usize),
    Empty(),
    EOF(),
}

pub enum FirstSet<'a> {
    Other(Symbol),
    Nonterminal(&'a HashSet<Symbol>),
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

fn set_has_nonterm(sets: &Vec<HashSet<Symbol>>) -> bool {
    for set in sets {
        for symbol in set {
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
    pub fn generate_firsts(&self) -> Vec<HashSet<Symbol>> {
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
                let (to_add, to_remove) = self.generate_first_nonterm(nonterminal, &firsts);
                for item in to_remove {
                    firsts[nonterminal].remove(&item);
                }
                for item in to_add {
                    firsts[nonterminal].insert(item);
                }
            }
            if !set_has_nonterm(&firsts) || iter > 100 {
                println!("broken iter={}", iter);
                break;
            }
        }
        //transform firsts into desired structure
        firsts
    }
    //generates the first set for a specific nonterminal from it's productions
    //this is the set of possible first terminals for a derviation of the nonterminal
    //this function is only one step of generate_firsts, it will not remove nonterminals in the resulting set
    fn generate_first_from_prods(
        &self,
        nonterminal: usize,
        mut firsts: Vec<HashSet<Symbol>>,
    ) -> Vec<HashSet<Symbol>> {
        let mut my_firsts = HashSet::new();
        for prod in &self.productions[nonterminal] {
            for symbol in &prod.rhs {
                match symbol {
                    Symbol::Empty() => {
                        my_firsts.insert(*symbol);
                    }
                    _ => {
                        my_firsts.insert(*symbol);
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
        firsts: &Vec<HashSet<Symbol>>,
    ) -> (Vec<Symbol>, Vec<Symbol>) {
        let mut to_add = Vec::new();
        let mut to_remove = Vec::new();
        for symbol in &firsts[nonterminal] {
            match symbol {
                Symbol::Nonterminal(x) => {
                    to_remove.push(*symbol);
                    if *x != nonterminal {
                        //don't want to add my first set to itself
                        for other_item in &firsts[*x] {
                            match other_item {
                                Symbol::Empty() => {
                                    //if the other nonterminal conatins the empty string, we need to add our own first set to it
                                    for my_item in &firsts[nonterminal] {
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
        firsts: &'a Vec<HashSet<Symbol>>,
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

    //generate the follow set for each nonterminal in a CFG.
    //example: if A-> b|c|De and D->d|<empty>, follow(D)={e,d}
    //return- first vec is indexed by nonterminal id, second vec contains list of terminals in the first set
    pub fn generate_follows(&self, firsts: &Vec<HashSet<Symbol>>) -> Vec<HashSet<Symbol>> {
        let mut follows = Vec::new();
        //populate follow sets
        for _ in 0..self.nonterminal_symbols.len() {
            follows.push(HashSet::new());
        }
        //1. first rule is always S' -> S <eof>, so follow(S') = <eof>
        follows[0].insert(Symbol::EOF());
        let mut keep_going = true;
        let mut iter = 0;
        while keep_going && iter < 100 {
            keep_going = false;
            //repeat these steps until no follow set grows
            //2. for each production A-> (stuff1)X(stuff2) ...
            //if <empty> is in first(stuff2), add [first(stuff2) union follow(A)] - <empty> to follow (x)
            //otherwise, add first(stuff2) to follow(X)
            for lhs in &self.productions {
                for production in lhs {
                    for index in 0..production.rhs.len() {
                        let symbol = production.rhs[index];
                        match symbol {
                            Symbol::Nonterminal(x) => {
                                let ret = self
                                    .add_follow_nonterminal(follows, firsts, production, x, index);
                                follows = ret.1;
                                if ret.0 {
                                    keep_going = true;
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }
            iter += 1;
        }
        println!("iter={}", iter);
        follows
    }

    //returns (bool: was anything added?, updated follow sets)
    fn add_follow_nonterminal(
        &self,
        mut follows: Vec<HashSet<Symbol>>,
        firsts: &Vec<HashSet<Symbol>>,
        production: &CFGProduction,
        nonterm_id: usize,
        symbol_index: usize,
    ) -> (bool, Vec<HashSet<Symbol>>) {
        let mut added = false;
        let beta_first = self.get_first(
            &production.rhs[(symbol_index + 1)..production.rhs.len()],
            firsts,
        );
        let mut add_lhs = false;
        match beta_first {
            Some(FirstSet::Nonterminal(set)) => {
                for symbol in set {
                    match symbol {
                        Symbol::Empty() => {
                            add_lhs = true;
                        }
                        _ => {
                            if follows[nonterm_id].insert(*symbol) {
                                added = true;
                            }
                        }
                    }
                }
            }
            Some(FirstSet::Other(symbol)) => match symbol {
                Symbol::Empty() => (),
                _ => {
                    if follows[nonterm_id].insert(symbol) {
                        added = true;
                    }
                }
            },
            None => add_lhs = true,
        }

        if add_lhs && production.nonterminal != nonterm_id {
            //brazy borrow checker stuff
            let prod: &mut std::collections::HashSet<Symbol>;
            let mine: &mut std::collections::HashSet<Symbol>;
            if production.nonterminal > nonterm_id {
                let tmp = follows.split_at_mut(production.nonterminal);
                prod = &mut tmp.1[0];
                mine = &mut tmp.0[nonterm_id];
            } else {
                let tmp = follows.split_at_mut(nonterm_id);
                prod = &mut tmp.0[production.nonterminal];
                mine = &mut tmp.1[0];
            }
            //have to add follow(LHS) to follow(symbol)
            for symbol in prod.iter() {
                match symbol {
                    Symbol::Empty() => (),
                    _ => {
                        if mine.insert(*symbol) {
                            added = true;
                        }
                    }
                }
            }
        }
        //added
        //
        (added, follows)
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
                    Some('<') if elem == "<eof>" => Symbol::EOF(),
                    Some('<') if elem == "<empty>" => Symbol::Empty(),
                    Some('.') => {
                        let n = CFG::register(&elem, &mut t_last, &mut t_map);
                        if n == t_last {
                            //register in terminal symbol table
                            t_symbols.push(elem);
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
                    print!(" {}", self.symbol_str(symbol));
                }
                println!();
            }
        }
    }
    //returns a reference to the symbol's string
    pub fn symbol_str(&self, symbol: &Symbol) -> &str {
        match symbol {
            Symbol::Terminal(x) => self.terminal_symbols[*x],
            Symbol::Nonterminal(x) => self.nonterminal_symbols[*x],
            Symbol::Empty() => "<empty>",
            Symbol::EOF() => "<eof>",
        }
    }
}
