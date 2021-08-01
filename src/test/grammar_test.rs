use super::super::*;

#[allow(dead_code)]
fn get_gstr() -> Vec<String> {
    let gstr = vec![
        "E -> E .+ T",
        "E -> T",
        "T -> T .* F",
        "T -> F",
        "F -> .( E .)",
        "F -> .id",
    ];
    gstr.iter().map(|&x| String::from(x)).collect()
}

#[allow(dead_code)]
fn get_cfg(grammar_strings: &Vec<String>) -> Box<grammar::CFG> {
    //let grammar_strings: Vec<String> = gstr.iter().map(|&x| String::from(x)).collect();
    Box::new(grammar::CFG::from_strings(&grammar_strings))
}

#[test]
fn test_first() {
    let firsts = get_cfg(&get_gstr()).generate_firsts();

    let first_s = &firsts[0];
    let first_e = &firsts[1];
    let first_t = &firsts[2];
    let first_f = &firsts[3];

    assert_eq!(2, first_s.len());
    assert_eq!(2, first_e.len());
    assert_eq!(2, first_t.len());
    assert_eq!(2, first_f.len());

    //.( and .id
    assert!(first_s.contains(&grammar::Symbol::Terminal(2)));
    assert!(first_s.contains(&grammar::Symbol::Terminal(4)));

    assert!(first_e.contains(&grammar::Symbol::Terminal(2)));
    assert!(first_e.contains(&grammar::Symbol::Terminal(4)));

    assert!(first_t.contains(&grammar::Symbol::Terminal(2)));
    assert!(first_t.contains(&grammar::Symbol::Terminal(4)));

    assert!(first_f.contains(&grammar::Symbol::Terminal(2)));
    assert!(first_f.contains(&grammar::Symbol::Terminal(4)));
}
#[test]
fn test_follow() {
    let gstr = &get_gstr();
    let cfg = get_cfg(gstr);
    let firsts = cfg.generate_firsts();
    let follows = cfg.generate_follows(&firsts);

    let follow_s = &follows[0];
    let follow_e = &follows[1];
    let follow_t = &follows[2];
    let follow_f = &follows[3];

    assert_eq!(1, follow_s.len());
    assert_eq!(3, follow_e.len());
    assert_eq!(4, follow_t.len());
    assert_eq!(4, follow_f.len());

    assert!(follow_s.contains(&grammar::Symbol::EOF()));

    //eof and .+ and .)
    assert!(follow_e.contains(&grammar::Symbol::EOF()));
    assert!(follow_e.contains(&grammar::Symbol::Terminal(0)));
    assert!(follow_e.contains(&grammar::Symbol::Terminal(3)));

    //eof and .+, .*, and .)
    assert!(follow_t.contains(&grammar::Symbol::EOF()));
    assert!(follow_t.contains(&grammar::Symbol::Terminal(0)));
    assert!(follow_t.contains(&grammar::Symbol::Terminal(1)));
    assert!(follow_t.contains(&grammar::Symbol::Terminal(3)));

    //eof and .+, .*, and .)
    assert!(follow_f.contains(&grammar::Symbol::EOF()));
    assert!(follow_f.contains(&grammar::Symbol::Terminal(0)));
    assert!(follow_f.contains(&grammar::Symbol::Terminal(1)));
    assert!(follow_f.contains(&grammar::Symbol::Terminal(3)));
}
