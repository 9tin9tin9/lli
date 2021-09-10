use crate::mem::Mem;
use crate::lex::Tok;
use super::*;

// source another file, load labels and symbols, don't execute
//      src: script_name(Sym)
pub fn src(v: &[Tok], _: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 1);
    let name = v[0].get_sym()?;
    Ok(Signal::Src(name.sym.clone()))
}
