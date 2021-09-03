use super::*;
use crate::lex::Tok;
use crate::mem::Mem;

pub fn jmp(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 1);
    let label = v[0].get_sym()?;
    let loc = m.label_find(label)?;
    Ok(Signal::Jmp(loc))
}

pub fn lbl(v: &[Tok], _: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 1);
    let label = v[0].get_sym()?;
    Ok(Signal::SetLbl(label.to_owned()))
}

pub fn skp(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 1);
    if v[0].get_value(m)? != 0.0 {
        Ok(Signal::Skp)
    }else{
        Ok(Signal::None)
    }
}

#[cfg(test)]
mod test;
