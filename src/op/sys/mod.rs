use crate::error::Error;
use crate::lex::Tok;
use crate::mem::{Mem, idx_incr};
use std::io::{ stdout, Write, stdin, Read };
use super::*;

const MAX_INPUT: usize = 1024;

pub fn write(v: &[Tok], m: &mut Mem) -> Result<Signal, Error> {
    argc_guard!(v, 2);
    let mut src_idx = v[0].get_loc(m)?;
    let size = v[1].get_value(m)?;
    if size != (size as usize) as f64 {
        return Err(Error::NegativeOrNonIntergerSize(size));
    }
    let mut stdout = stdout();
    let mut bytes_wrote = 0;
    for _ in 0..size as usize {
        stdout.write(&[m.mem_at(src_idx)? as u8]).unwrap();
        idx_incr(&mut src_idx, 1);
        bytes_wrote += 1;
    }
    m.mem_set(0, bytes_wrote as f64)?;
    Ok(Signal::None)
}

pub fn print_num(v: &[Tok], m: &mut Mem) -> Result<Signal, Error> {
    argc_guard!(v, 1);
    let val = v[0].get_value(m)?;
    print!("{}", val);
    Ok(Signal::None)
}

pub fn read(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 2);
    let des_idx = v[0].get_loc(m)?;
    let size = v[1].get_value(m)?;
    if size != (size as usize) as f64 {
        return Err(Error::NegativeOrNonIntergerSize(size));
    }
    let size = size as usize;
    let mut stdin = stdin();
    let mut buf = [0; MAX_INPUT];
    stdin.read(&mut buf).unwrap();
    for i in 0..MAX_INPUT {
        let c = buf[i] as f64;
        if c == 0.0 || i == size {
            m.mem_set(des_idx + i as isize, 0.0)?;
            m.mem_set(0, i as f64)?;
            break;
        }
        m.mem_set(des_idx + i as isize, c)?;
    }
    Ok(Signal::None)
}
