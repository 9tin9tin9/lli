use crate::lex::Tok;
use crate::mem::Mem;
use crate::error::Error;

#[test]
fn and(){
    let v = vec![Tok::Num(1.0), Tok::Num(10.0)];
    let mut m = Mem::new();
    super::and(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), 1.0);
}

#[test]
fn and_false(){
    let v = vec![Tok::Num(0.0), Tok::Num(10.0)];
    let mut m = Mem::new();
    super::and(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), 0.0);
}

#[test]
fn or(){
    let v = vec![Tok::Num(1.0), Tok::Num(10.0)];
    let mut m = Mem::new();
    super::or(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), 1.0);
    let v = vec![Tok::Num(0.0), Tok::Num(10.0)];
    super::or(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), 1.0);
}

#[test]
fn or_false(){
    let v = vec![Tok::Num(0.0), Tok::Num(0.0)];
    let mut m = Mem::new();
    super::and(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), 0.0);
}

#[test]
fn and_or_argc(){
    let v = vec![Tok::Num(0.0)];
    let mut m = Mem::new();
    let r = super::and(&v, &mut m);
    assert_eq!(r, Err(Error::WrongArgCount(2, 1)));
}

#[test]
fn not(){
    let v = vec![Tok::Num(0.0)];
    let mut m = Mem::new();
    super::not(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), 1.0);
}

#[test]
fn not_false(){
    let v = vec![Tok::Num(10.0)];
    let mut m = Mem::new();
    super::not(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), 0.0);
}
