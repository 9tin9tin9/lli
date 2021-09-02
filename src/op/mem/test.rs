use crate::lex::Tok;
use crate::mem::Mem;

#[test]
fn mov(){
    let v = vec![Tok::Idx(10), Tok::Num(100.0)];
    let mut m = Mem::new();
    super::mov(&v, &mut m).unwrap();
    assert_eq!(
        m.mem_at(10).unwrap(),
        100.0);
}

#[test]
fn cpy(){
    let v = vec![Tok::Idx(1), Tok::Ltl("asdasd".to_string()), Tok::Num(6.0)];
    let mut m = Mem::new();
    super::cpy(&v, &mut m).unwrap();
    assert_eq!(
        m.read_ltl(1).unwrap(),
        "asdasd");
}

#[test]
fn var(){
    let v = vec![Tok::Sym("A".to_string()), Tok::Idx(10)];
    let mut m = Mem::new();
    super::var(&v, &mut m).unwrap();
    assert_eq!(m.var_find("A").unwrap(), 10);
}

#[test]
fn incr(){
    let v = vec![Tok::Sym("A".to_string()), Tok::Num(10.0)];
    let mut m = Mem::new();
    m.var_add("A".to_string(), 0);
    super::incr(&v, &mut m).unwrap();
    assert_eq!(m.var_find("A").unwrap(), 10);
    m.var_add("A".to_string(), -1);
    super::incr(&v, &mut m).unwrap();
    assert_eq!(m.var_find("A").unwrap(), -11);
}

#[test]
fn decr(){
    let v = vec![Tok::Sym("A".to_string()), Tok::Num(10.0)];
    let mut m = Mem::new();
    m.var_add("A".to_string(), 0);
    super::decr(&v, &mut m).unwrap();
    assert_eq!(m.var_find("A").unwrap(), -10);
    m.var_add("A".to_string(), -1);
    super::decr(&v, &mut m).unwrap();
    assert_eq!(m.var_find("A").unwrap(), 9);
}

#[test]
fn allc(){
    let v = vec![Tok::Num(100.0)];
    let mut m = Mem::new();
    let o_size = m.pmem_len();
    super::allc(&v, &mut m).unwrap();
    assert_eq!(m.pmem_len()-o_size, 100);
}
