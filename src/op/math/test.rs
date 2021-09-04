use crate::lex::Tok;
use crate::mem::Mem;
use crate::error::Error;

#[test]
fn add(){
    let v = vec![Tok::Num(1.0), Tok::Num(2.0)];
    let mut m = Mem::new();
    super::add(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), 3.0);
}

#[test]
fn sub(){
    let v = vec![Tok::Num(1.0), Tok::Num(2.0)];
    let mut m = Mem::new();
    super::sub(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), -1.0);
}

#[test]
fn mul(){
    let v = vec![Tok::Num(1.0), Tok::Num(2.0)];
    let mut m = Mem::new();
    super::mul(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), 2.0);
}

#[test]
fn div(){
    let v = vec![Tok::Num(1.0), Tok::Num(2.0)];
    let mut m = Mem::new();
    super::div(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), 0.5);
}

#[test]
fn r#mod(){
    let v = vec![Tok::Num(11.0), Tok::Num(10.0)];
    let mut m = Mem::new();
    super::r#mod(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), 1.0);
}

#[test]
fn div_by_zero(){
    let v = vec![Tok::Num(1.0), Tok::Num(0.0)];
    let mut m = Mem::new();
    super::div(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap(), f64::INFINITY);
}

#[test]
fn div_zero_by_zero(){
    let v = vec![Tok::Num(0.0), Tok::Num(0.0)];
    let mut m = Mem::new();
    super::div(&v, &mut m).unwrap();
    assert_eq!(m.mem_at(0).unwrap().is_nan(), true);
}

#[test]
fn add_incorrect_args_num(){
    let v = vec![Tok::Ltl("asdas".to_string()), Tok::Num(0.0), Tok::Ltl("asd".to_string())];
    let mut m = Mem::new();
    let r = super::add(&v, &mut m);
    assert_eq!(Err(Error::WrongArgCount(2, 3)), r);
}

#[test]
fn add_incorrect_args_type(){
    let v = vec![Tok::Ltl("asdas".to_string()), Tok::Ltl("asdasd".to_string())];
    let mut m = Mem::new();
    let r = super::add(&v, &mut m);
    assert_eq!(
        Err(Error::WrongArgType(
            vec![Tok::NUM_STR, Tok::IDX_STR, Tok::VAR_STR], 
            Tok::LTL_STR)), r);
}
