use super::*;

#[test]
fn tokenize_sym(){
    assert_eq!(
        vec![Tok::Sym("A".to_string()), 
             Tok::Sym("b".to_string()), 
             Tok::Sym("c".to_string()), 
             Tok::Sym("d".to_string())],
        tokenize(&"A: b, c, d".to_string()).unwrap());
}

#[test]
fn tokenize_num(){
    assert_eq!(
        vec![Tok::Sym("A".to_string()), 
             Tok::Num(-10.0234f64), 
             Tok::Sym("c".to_string())],
        tokenize(&"A: -10.0234 , c".to_string()).unwrap());
    assert_ne!(
        Ok(Vec::new()),
        tokenize(&"A: -10.02.34 , c".to_string()));
}

#[test]
fn tokenize_strltl(){
    assert_eq!(
        vec![Tok::Sym("A".to_string()), 
             Tok::Ltl("asd\\\"asd".to_string()), 
             Tok::Sym("c".to_string())],
        tokenize(&"A: \"asd\\\"asd\" , c".to_string()).unwrap());
}

#[test]
fn tokenize_idx(){
    assert_eq!(
        vec![Tok::Sym("Aasdasd".to_string()), 
             Tok::Idx(-123), 
             Tok::Sym("casdasd".to_string())],
        tokenize(&"Aasdasd : [-123] ,casdasd".to_string()).unwrap());
    assert_ne!(
        Ok(Vec::new()),
        tokenize(&"A : [asd], cd".to_string()));
    assert_ne!(
        Ok(Vec::new()),
        tokenize(&"A : [-123.1] , cd".to_string()));
}

#[test]
fn tokenize_var(){
    assert_eq!(
        vec![Tok::Sym("Aasdasd".to_string()), 
             Tok::Var("asd".to_string()), 
             Tok::Sym("casdasd".to_string())],
        tokenize(&"Aasdasd : $asd , casdasd".to_string()).unwrap());
    assert_eq!(
        vec![Tok::Sym("A".to_string()), 
             Tok::Var("123".to_string()), 
             Tok::Sym("casdasd".to_string())],
        tokenize(&"A : $123 , casdasd".to_string()).unwrap());
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
        vec![Tok::Sym("asd".to_string())],
        tokenize(&" asd".to_string()).unwrap());
}

#[test]
fn tokenize_op_err(){
    assert_eq!(
        Err("Expects symbol as operator".to_string()),
        tokenize(&"123".to_string()));
    assert_eq!(
        Err("Expects symbol as operator".to_string()),
        tokenize(&"[-123]".to_string()));
    assert_eq!(
        Err("Expects symbol as operator".to_string()),
        tokenize(&"$asd".to_string()));
}

#[test]
fn tokenize_non_delim_after_sym_end(){
    assert_eq!(
        Err("Found non-delimeter after symbol ends".to_string()),
        tokenize(&"asd 2".to_string()));
}

#[test]
fn tokenize_unexpected(){
    assert_eq!(
        Err("Unexpected ','".to_string()),
        tokenize(&"asd , asd".to_string()));
    assert_eq!(
        Err("Unexpected ':'".to_string()),
        tokenize(&"asd : asd:".to_string()));
}
