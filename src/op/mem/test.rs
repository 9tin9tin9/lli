use crate::lex::*;
use crate::mem::Mem;

#[test]
fn mov(){
    let v = vec![Tok::Idx(Idx::Num(10)), Tok::Num(100.0)];
    let mut m = Mem::new();
    m.pmem_allc(&[0.0; 10]);
    super::mov(&v, &mut m).unwrap();
    assert_eq!(
        m.mem_at(10).unwrap(),
        100.0);
}

#[test]
fn cpy(){
    let v = vec![Tok::Idx(Idx::Num(1)), Tok::Ltl("asdasd".to_string()), Tok::Num(6.0)];
    let mut m = Mem::new();
    m.pmem_allc(&[0.0; 8]);
    super::cpy(&v, &mut m).unwrap();
    assert_eq!(
        m.read_ltl(1).unwrap(),
        "asdasd");
}

#[test]
fn var(){
    let v = vec![Tok::Sym(HashIdx::new("A", 0)), Tok::Idx(Idx::Num(10))];
    let mut m = Mem::new();
    m.var_add(10);
    super::var(&v, &mut m).unwrap();
    if let Tok::Sym(hi) = &v[0] {
        assert_eq!(m.var_find(&hi).unwrap(), 10);
    }
}

#[test]
fn incr(){
    let v = vec![Tok::Var(HashIdx::new("A", 0)), Tok::Num(10.0)];
    let v2 = vec![Tok::Var(HashIdx::new("A", 0)), Tok::Num(-8.0)];
    let mut m = Mem::new();
    if let Tok::Sym(hi) = &v[0]{
        m.var_add(0);
        super::incr(&v, &mut m).unwrap();
        assert_eq!(m.var_find(&hi).unwrap(), 10);
        m.var_set(hi.idx, -1);
        super::incr(&v, &mut m).unwrap();
        assert_eq!(m.var_find(&hi).unwrap(), -11);
        m.var_set(hi.idx, 10);
        super::incr(&v2, &mut m).unwrap();
        assert_eq!(m.var_find(&hi).unwrap(), 2);
    }
}

#[test]
fn allc(){
    let v = vec![Tok::Num(100.0)];
    let mut m = Mem::new();
    let o_size = m.pmem_len();
    super::allc(&v, &mut m).unwrap();
    assert_eq!(m.pmem_len()-o_size, 100);
}
