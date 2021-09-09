use crate::mem::Mem;
use crate::lex::Tok;
use super::*;

pub fn src(v: &[Tok], _: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 1);
    let name = v[0].get_sym()?;
    Ok(Signal::Src(name.sym.clone()))
}
