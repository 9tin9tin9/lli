use super::*;
use crate::lex::Tok;
use crate::mem::Mem;

pub fn jmp(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 2);
    if v[0].get_value(m)? != 0.0 {
        let label = v[1].get_sym()?;
        let loc = m.label_find(label)?;
        Ok(Signal::Jmp(loc))
    }else{
        Ok(Signal::None)
    }
}

pub fn lbl(v: &[Tok], _: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 1);
    let label = v[0].get_sym()?;
    Ok(Signal::SetLbl(label.idx))
}

pub fn ret(v: &[Tok], _: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 0);
    Ok(Signal::Ret)
}

#[cfg(test)]
mod test;
