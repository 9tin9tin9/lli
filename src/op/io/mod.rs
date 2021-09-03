use crate::error::Error;
use crate::lex::Tok;
use crate::mem::{Mem, idx_incr};
use std::io::{ stdout, Write };
use super::*;

pub fn out(v: &[Tok], m: &mut Mem) -> Result<Signal, Error> {
    argc_guard!(v, 2);
    let mut src_idx = v[0].get_loc(m)?;
    let size = v[1].get_value(m)?;
    if size != (size as usize) as f64 {
        return Err(Error::NegativeOrNonIntergerSize(size));
    }
    let mut stdout = stdout();
    for _ in 0..size as usize {
        stdout.write(&[m.mem_at(src_idx)? as u8]).unwrap();
        idx_incr(&mut src_idx, 1);
    }
    Ok(Signal::None)
}

pub fn outa(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 1);
    let val = v[0].get_value(m)? as u8;
    let mut stdout = stdout();
    stdout.write(&[val]).unwrap();
    Ok(Signal::None)
}

pub fn outv(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 1);
    let val = v[0].get_value(m)?;
    print!("{}", val);
    Ok(Signal::None)
}
