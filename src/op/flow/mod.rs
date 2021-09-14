use super::*;
use crate::lex::Tok;
use crate::mem::Mem;

// unconditional jump
//      jmp: lbl(Sym)
pub fn jmp(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 1);
    let label = v[0].get_sym()?;
    let loc = m.label_find(label)?;
    Ok(Signal::Jmp(loc))
}

// jump if cond is true
//      jc: cond(Value), lbl(Sym)
pub fn jc(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 2);
    let cond = v[0].get_value(m)?;
    if cond != 0.0 {
        let label = v[1].get_sym()?;
        let loc = m.label_find(label)?;
        Ok(Signal::Jmp(loc))
    }else{
        Ok(Signal::None)
    }
}

// Set label.
// Label symbol created during preprocess
// This function updates the line number the label points to
//      lbl: lbl(Sym)
pub fn lbl(v: &[Tok], _: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 1);
    let label = v[0].get_sym()?;
    Ok(Signal::SetLbl(label.idx))
}

// Set alias.
// Act as function pointer
// Allowing changing label during run time to know label locations
pub fn als(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 1);
    let alias = v[0].get_sym()?;
    let label = v[1].get_sym()?;
    let loc = m.label_find(label)?;
    Ok(Signal::SetAls(alias.idx, loc))
}

#[cfg(test)]
mod test;
