use crate::error::Error;
use crate::lex::Tok;
use crate::mem::Mem;
use super::*;

pub fn print_num(v: &[Tok], m: &mut Mem) -> Result<Signal, Error> {
    argc_guard!(v, 1);
    let val = v[0].get_value(m)?;
    print!("{}", val);
    Ok(Signal::None)
}
