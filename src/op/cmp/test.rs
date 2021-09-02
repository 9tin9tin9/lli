use crate::lex::Tok;
use crate::mem::Mem;

#[test]
fn eq(){
    let v = vec![Tok::Num(1.0), Tok::Num(1.0)];
    let mut m = Mem::new();
    super::eq(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), 1.0);
}

#[test]
fn ne(){
    let v = vec![Tok::Num(1.0), Tok::Num(1.0)];
    let mut m = Mem::new();
    super::ne(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), 0.0);
}

#[test]
fn gt(){
    let v = vec![Tok::Num(1.0), Tok::Num(1.0)];
    let mut m = Mem::new();
    super::gt(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), 0.0);
    let v = vec![Tok::Num(2.0), Tok::Num(1.0)];
    super::gt(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), 1.0);
}

#[test]
fn lt(){
    let v = vec![Tok::Num(1.0), Tok::Num(1.0)];
    let mut m = Mem::new();
    super::lt(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), 0.0);

    let v = vec![Tok::Num(0.0), Tok::Num(1.0)];
    super::lt(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), 1.0);
}
