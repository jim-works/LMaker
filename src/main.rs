mod grammar;
mod parse_table;

fn main() {
    let gstr = vec![
        "E -> E .+ T",
        "E -> T",
        "T -> T .* F",
        "T -> F",
        "F -> .lparen E .rparen",
        "F -> .id",
    ];
    let grammar_strings: Vec<String> = gstr.iter().map(|&x| String::from(x)).collect();
    let cfg = grammar::CFG::from_strings(&grammar_strings);
    parse_table::generate_table_slr(&cfg);
}
