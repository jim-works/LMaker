mod grammar;
mod lr1_generator;
mod parse_table;

//TODO: FIX EMPTIES

fn main() {
    let gstr = vec![
        "E -> . E .+ T",
        "E -> T",
        "T -> T .* F",
        "T -> F",
        "F -> .lparen E .rparen",
        "F -> .id",
    ];
    let grammar_strings: Vec<String> = gstr.iter().map(|&x| String::from(x)).collect();
    let cfg = grammar::CFG::from_strings(&grammar_strings);
    cfg.print();
    println!("\nFirsts:");
    let firsts = cfg.generate_firsts();
    for index in 0..firsts.len() {
        print!("first({}) = ", cfg.nonterminal_symbols[index]);
        for symbol in &firsts[index] {
            match symbol {
                grammar::Symbol::Terminal(t) => print!("{} ", cfg.terminal_symbols[*t]),
                grammar::Symbol::Nonterminal(nt) => print!("{} ", cfg.nonterminal_symbols[*nt]),
                grammar::Symbol::Empty() => print!("<empty> "),
                grammar::Symbol::EOF() => print!("<eof> "),
            }
        }
        println!();
    }
}
