use crate::lex::Tok;
use crate::mem::*;
use super::*;
use crate::error::Error;

// Assignment, read value
//      mov: des(WPtr), src(Value)
pub fn mov(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 2);
    let src_val = v[1].get_value(&*m)?;
    v[0].write_value(m, src_val)?;
    Ok(Signal::None)
}

// Memcpy. When src = Ltl, a new ltl is created and its idx is used as src idx
//      cpy: des(WPtr), src(Ptr), size(Value)
pub fn cpy(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 3);
    let mut des_idx = v[0].get_loc(m)?;
    let mut src_idx = v[1].get_loc(m)?;
    let size_val = v[2].get_uint(m)?;
    for _ in 0..size_val {
        Tok::Idx(des_idx).write_value(m, m.mem_at(src_idx).unwrap())?;
        mem::idx_incr(&mut des_idx, 1);
        mem::idx_incr(&mut src_idx, 1);
    }
    Ok(Signal::None)
}

// Creates or update $name with index = idx
//      var: name(Sym), idx(Ptr)
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
        let incr_val = $v[1].get_uint($m)?;
        let mut var_idx = $m.var_find(&var)?;
        $a(&mut var_idx, incr_val as isize);
        // also update var_idx of the variable
        $m.var_set(var.idx, var_idx);
        return Ok(Signal::None);
    }
}

// Used to iterate->read/write pmem, potentially can be used to do stack operations
//      incr: var(Var), num(Value)
pub fn incr(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    mut_var_idx!(v, m, idx_incr);
}

// Used to iterate->read/write pmem, potentially can be used to do stack operations
//      decr: var(Var), num(Value)
pub fn decr(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    mut_var_idx!(v, m, idx_decr);
}

// Push slots to pmem
//      allc: size(Value)
pub fn allc(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 1);
    let size = v[0].get_uint(&m)?;
    m.pmem_allc(&vec![0f64; size as usize]);
    m.mem_set(0, size as f64)?;
    Ok(Signal::None)
}

#[cfg(test)]
mod test;
