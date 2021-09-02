use crate::lex::Tok;
use crate::mem::Mem;
use crate::code::Code;
use crate::error::Error;

#[test]
fn parse_statement_lookup_op(){
    let t = vec![Tok::Sym("nop".to_string())];
    let mut m = Mem::new();
    let mut c = Code::new();
    c.push(t);
    assert_eq!(super::exec(&mut m, &c).unwrap(), super::Signal::None);
}

#[test]
fn tok_read_value(){
    let t = Tok::Num(10.0);
    let mut m = Mem::new();
    assert_eq!(t.get_value(&m).unwrap(), 10f64);
    let t = Tok::Idx(1);
    m.pmem_set(1, 10.0).unwrap();
    assert_eq!(t.get_value(&m).unwrap(), 10f64);
    let t = Tok::Var("A".to_string());
    m.var_add("A".to_string(), 1);
    assert_eq!(t.get_value(&m).unwrap(), 10f64);
}

#[test]
fn tok_read_value_invalid_memory_access(){
    let t = Tok::Idx(-1);
    let m = Mem::new();
    assert_eq!(t.get_value(&m), Err(Error::InvalidMemAccess(-1)));
}

#[test]
fn tok_read_value_wrong_type(){
    let t = Tok::Eof;
    let m = Mem::new();
    assert_eq!(t.get_value(&m), 
        Err(Error::WrongArgType(
                vec![Tok::NUM_STR, Tok::IDX_STR, Tok::VAR_STR],
                Tok::EOF_STR)));
}

#[test]
fn tok_get_loc(){
    let t = Tok::Idx(100);
    let mut m = Mem::new();
    assert_eq!(t.get_loc(&mut m).unwrap(), 100);
    let t = Tok::Var("A".to_string());
    m.var_add("A".to_string(), 100);
    assert_eq!(t.get_loc(&mut m).unwrap(), 100);
    let t = Tok::Ltl("asda".to_string());
    assert_eq!(t.get_loc(&mut m).unwrap(), -1);
}

#[test]
fn tok_get_loc_wrong_type(){
    let t = Tok::Eof;
    let mut m = Mem::new();
    assert_eq!(
        t.get_loc(&mut m), 
        Err(Error::WrongArgType(
                vec![Tok::IDX_STR, Tok::VAR_STR, Tok::LTL_STR],
                Tok::EOF_STR)));
}

#[test]
fn tok_create_ltl(){
    let t = Tok::Ltl("asd".to_string());
    let mut m = Mem::new();
    assert_eq!(t.create_ltl(&mut m).unwrap(), -1);
    assert_eq!(m.read_ltl(-1).unwrap(), "asd");
}

#[test]
fn tok_create_ltl_wrong_type(){
    let t = Tok::Eof;
    let mut m = Mem::new();
    assert_eq!(
        t.create_ltl(&mut m),
        Err(Error::WrongArgType(
                vec![Tok::LTL_STR],
                Tok::EOF_STR)));
}
