use super::super::*;

#[allow(dead_code)]
fn get_gstr() -> Vec<String> {
    let gstr = vec![
        "S -> S .; A",
        "S -> A",
        "A -> E",
        "A -> .id .:= E",
        "E -> E .+ .id",
        "E -> .id",
    ];
    gstr.iter().map(|&x| String::from(x)).collect()
}

#[allow(dead_code)]
fn get_cfg(grammar_strings: &Vec<String>) -> grammar::CFG {
    grammar::CFG::from_strings(&grammar_strings)
}

#[test]
fn closure() {
    /*
    S' -> S <eof> , <eof>
    S -> A , .;
    A -> E , .;
    E -> .id , <eof>
    A -> .id .:= E , .;
    S -> A , <eof>
    E -> E .+ .id , .;
    S -> S .; A , <eof>
    A -> .id .:= E , <eof>
    S -> S .; A , .;
    E -> E .+ .id , <eof>
    E -> .id , .;
    E -> .id , .+
    E -> E .+ .id , .+
    A -> E , <eof>
        */
}
