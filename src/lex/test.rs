use super::*;

#[test]
fn tokenize_sym(){
    assert_eq!(
        vec![Tok::Sym(HashIdx::from_str("A")), 
             Tok::Sym(HashIdx::from_str("b")), 
             Tok::Sym(HashIdx::from_str("c")), 
             Tok::Sym(HashIdx::from_str("d"))],
        tokenize(&"A: b, c, d ".to_string()).unwrap());
}

#[test]
fn tokenize_num(){
    assert_eq!(
        vec![Tok::Sym(HashIdx::from_str("A")), 
             Tok::Num(-10.0234f64), 
             Tok::Sym(HashIdx::from_str("c"))],
        tokenize(&"A: -10.0234 , c ".to_string()).unwrap());
    assert_ne!(
        Ok(Vec::new()),
        tokenize(&"A: -10.02.34 , c ".to_string()));
}

#[test]
fn tokenize_strltl(){
    assert_eq!(
        vec![Tok::Sym(HashIdx::from_str("A")), 
             Tok::Ltl("asd \"asd".to_string()), 
             Tok::Sym(HashIdx::from_str("c"))],
        tokenize(&"A: \"asd \\\"asd\" , c ".to_string()).unwrap());
}

#[test]
fn tokenize_idx(){
    assert_eq!(
        vec![Tok::Sym(HashIdx::from_str("Aasdasd")), 
             Tok::Idx(-123), 
             Tok::Sym(HashIdx::from_str("casdasd"))],
        tokenize(&"Aasdasd : [-123] ,casdasd ".to_string()).unwrap());
    assert_ne!(
        Ok(Vec::new()),
        tokenize(&"A : [asd], cd ".to_string()));
    assert_ne!(
        Ok(Vec::new()),
        tokenize(&"A : [-123.1] , cd ".to_string()));
}

#[test]
fn tokenize_var(){
    assert_eq!(
        vec![Tok::Sym(HashIdx::from_str("Aasdasd")), 
             Tok::Var(HashIdx::from_str("asd")), 
             Tok::Sym(HashIdx::from_str("casdasd"))],
        tokenize(&"Aasdasd : $asd , casdasd ".to_string()).unwrap());
    assert_eq!(
        vec![Tok::Sym(HashIdx::from_str("A")),
             Tok::Var(HashIdx::from_str("123")), 
             Tok::Sym(HashIdx::from_str("casdasd"))],
        tokenize(&"A : $123 , casdasd ".to_string()).unwrap());
}

#[test]
fn tokenize_empty_token(){
    assert_eq!(
        Err("Empty token. Expects operator".to_string()),
        tokenize(&" : ".to_string()));
    assert_eq!(
        Err("Empty token. Expects argument".to_string()),
        tokenize(&"A : , ".to_string()));
}

#[test]
fn tokenize_op_only(){
    assert_eq!(
        vec![Tok::Sym(HashIdx::from_str("asd"))],
        tokenize(&" asd ".to_string()).unwrap());
}

#[test]
fn tokenize_op_err(){
    assert_eq!(
        Err("Expects symbol as operator".to_string()),
        tokenize(&"123 ".to_string()));
    assert_eq!(
        Err("Expects symbol as operator".to_string()),
        tokenize(&"[-123] ".to_string()));
    assert_eq!(
        Err("Expects symbol as operator".to_string()),
        tokenize(&"$asd ".to_string()));
}

#[test]
fn tokenize_non_delim_after_sym_end(){
    assert_eq!(
        Err("Found non-delimeter after symbol ends".to_string()),
        tokenize(&"asd 2 ".to_string()));
}

#[test]
fn tokenize_unexpected(){
    assert_eq!(
        Err("Unexpected ','".to_string()),
        tokenize(&"asd , asd ".to_string()));
    assert_eq!(
        Err("Unexpected ':'".to_string()),
        tokenize(&"asd : asd: ".to_string()));
}

#[test]
fn read_value(){
    let t = Tok::Num(10.0);
    let mut m = Mem::new();
    assert_eq!(t.get_value(&m).unwrap(), 10f64);
    let t = Tok::Idx(1);
    m.pmem_set(1, 10.0).unwrap();
    assert_eq!(t.get_value(&m).unwrap(), 10f64);
    let t = Tok::Var(HashIdx::new("A", 0));
    m.var_add(1);
    assert_eq!(t.get_value(&m).unwrap(), 10f64);
}

#[test]
fn read_value_invalid_memory_access(){
    let t = Tok::Idx(-1);
    let m = Mem::new();
    assert_matches!(t.get_value(&m), Err(Error::InvalidMemAccess(-1)));
}

#[test]
fn read_value_wrong_type(){
    let t = Tok::Eof;
    let m = Mem::new();
    let got = t.get_value(&m).unwrap_err();
    let expected = Error::WrongArgType(
                vec![Tok::NUM_STR, Tok::IDX_STR, Tok::VAR_STR],
                Tok::EOF_STR);
    assert_matches!(got, expected)
}

#[test]
fn get_loc(){
    let t = Tok::Idx(100);
    let mut m = Mem::new();
    assert_eq!(t.get_loc(&mut m).unwrap(), 100);
    let t = Tok::Var(HashIdx::new("A", 0));
    m.var_add(100);
    assert_eq!(t.get_loc(&mut m).unwrap(), 100);
    let t = Tok::Ltl("asda".to_string());
    assert_eq!(t.get_loc(&mut m).unwrap(), -1);
}

#[test]
fn get_loc_wrong_type(){
    let t = Tok::Eof;
    let mut m = Mem::new();
    let got = t.get_loc(&mut m);
    let expected = Error::WrongArgType(
                vec![Tok::IDX_STR, Tok::VAR_STR, Tok::LTL_STR],
                Tok::EOF_STR);
    assert_matches!(got, expected);
}

#[test]
fn create_ltl(){
    let t = Tok::Ltl("asd".to_string());
    let mut m = Mem::new();
    assert_eq!(t.create_ltl(&mut m).unwrap(), -1);
    assert_eq!(m.read_ltl(-1).unwrap(), "asd");
}

#[test]
fn create_ltl_wrong_type(){
    let t = Tok::Eof;
    let mut m = Mem::new();
    let got = t.create_ltl(&mut m);
    let expected = Error::WrongArgType(
                vec![Tok::LTL_STR],
                Tok::EOF_STR);
    assert_matches!(got, expected);
}
