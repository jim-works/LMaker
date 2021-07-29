mod grammar;
mod lr1_generator;
mod parse_table;

//TODO: FIX EMPTIES
//      fix follows lol

fn main() {
    let gstr = vec!["S -> C C", "C -> .e C", "C -> .d"];
    let grammar_strings: Vec<String> = gstr.iter().map(|&x| String::from(x)).collect();
    let cfg = grammar::CFG::from_strings(&grammar_strings);
    cfg.print();

    println!("\nFirsts:");
    let firsts = cfg.generate_firsts();
    for index in 0..firsts.len() {
        print!("first({}) = ", cfg.nonterminal_symbols[index]);
        for symbol in &firsts[index] {
            print!(" {}", cfg.symbol_str(symbol));
        }
        println!();
    }

    println!("\nFollows:");
    let follows = cfg.generate_follows(&firsts);
    for index in 0..follows.len() {
        print!("follow({}) = ", cfg.nonterminal_symbols[index]);
        for symbol in &follows[index] {
            print!(" {}", cfg.symbol_str(symbol));
        }
        println!();
    }
}
