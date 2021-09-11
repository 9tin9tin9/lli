use ahash::AHashMap;
use crate::lex::*;
use crate::mem::Mem;
use crate::code::Code;

#[test]
fn parse_statement_lookup_op(){
    let t = vec![Tok::Sym(HashIdx::from_str("nop"))];
    let mut m = Mem::new();
    let mut c = Code::new();
    let mut op_idx_table: AHashMap<&'static str, usize> = AHashMap::new();
    let mut func_vec: Vec<super::OpFunc> = Vec::new();
    super::init_op_table(&mut op_idx_table, &mut func_vec);
    crate::preprocess(&op_idx_table, &mut m, &mut c, t).unwrap();
    assert_eq!(super::exec(&mut func_vec, &mut m, &c).unwrap(), super::Signal::None);
}
