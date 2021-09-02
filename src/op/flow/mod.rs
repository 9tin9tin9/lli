use super::*;
use crate::lex::Tok;
use crate::mem::Mem;

pub fn jmp(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 1);
    let label = v[0].get_sym()?;
    let loc = m.label_find(&label)?;
    Ok(Signal::Jmp(loc))
}

pub fn lbl(v: &[Tok], _: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 1);
    let label = v[0].get_sym()?;
    Ok(Signal::SetLbl(label.to_string()))
}

pub fn cnd(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!{v, 2};
    let true_label = v[0].get_sym()?;
    let false_label = v[1].get_sym()?;
    let is_true = m.mem_at(0)? == 0.0;
    let loc = m.label_find(if is_true {
        &true_label
    }else {
        &false_label
    })?;
    Ok(Signal::Jmp(loc))
}

#[cfg(test)]
mod test;
