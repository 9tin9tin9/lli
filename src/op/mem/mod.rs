use crate::lex::Tok;
use crate::mem::*;
use super::*;
use crate::error::Error;

pub fn mov(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 2);
    let src_val = v[1].get_value(&*m)?;
    v[0].write_value(m, src_val)?;
    Ok(Signal::None)
}

pub fn cpy(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 3);
    let mut des_idx = v[0].get_loc(m)?;
    let mut src_idx = v[1].get_loc(m)?;
    let size_val = v[2].get_uint(m)?;
    for _ in 0..size_val {
        Tok::Idx(des_idx).write_value( m, m.mem_at(src_idx).unwrap())?;
        mem::idx_incr(&mut des_idx, 1);
        mem::idx_incr(&mut src_idx, 1);
    }
    Ok(Signal::None)
}

pub fn var(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 2);
    let var = v[0].get_sym()?;
    let idx = v[1].get_loc(m)?;
    m.var_set(var.idx, idx);
    Ok(Signal::None)
}

macro_rules! mut_var_idx {
    ( $v:expr, $m:expr, $a:ident ) => {
        argc_guard!($v, 2);
        let var = $v[0].get_sym()?;
        let incr_val = $v[1].get_value($m)?;
        let mut var_idx = $m.var_find(&var)?;
        $a(&mut var_idx, incr_val as isize);
        $m.var_set(var.idx, var_idx);
        return Ok(Signal::None);
    }
}

pub fn incr(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    mut_var_idx!(v, m, idx_incr);
}

pub fn decr(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    mut_var_idx!(v, m, idx_decr);
}

pub fn allc(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 1);
    let size = v[0].get_value(&m)? as usize;
    m.pmem_allc(&vec![0f64; size]);
    m.mem_set(0, size as f64)?;
    Ok(Signal::None)
}

#[cfg(test)]
mod test;
