use crate::lex::Tok;
use crate::mem::Mem;
use crate::op::Signal;
use crate::error::Error;

pub fn nop(_: &[Tok], _: &mut Mem) -> Result<Signal, Error>{
    Ok(Signal::None)
}
