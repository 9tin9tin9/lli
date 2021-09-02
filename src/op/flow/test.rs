use crate::lex::Tok;
use crate::mem::Mem;
use crate::code::Code;
use super::*;

#[test]
fn jmp(){
    let mut code = Code::new();
    let mut mem = Mem::new();
    let v1 = vec![Tok::Sym("L".to_string())];
    let v2 = vec![Tok::Num(10.0), Tok::Num(10.0)];
    code.push(v1.clone());
    code.push(v2.clone());
    mem.label_add("L".to_string(), 0);
    let l = if let Signal::Jmp(l) = super::jmp(&v1, &mut mem).unwrap() {
        l
    }else{
        panic!("Not returning Signal::Jmp");
    };
    assert_eq!(l, 0);
}
