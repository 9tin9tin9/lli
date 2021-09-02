use crate::mem::Mem;
use crate::lex::Tok;
use super::*;

fn parse_arg(v: &[Tok], m: &Mem) -> Result<(f64, f64), Error>{
    argc_guard!(v, 2);
    let mut a = [0.0; 2];
    for i in 0..2 {
        a[i] = v[i].get_value(m)?;
    }
    Ok((a[0], a[1]))
}

macro_rules! math {
    ( $o:tt, $v:expr, $m:expr ) => {
        {
            let (left, right) = parse_arg($v, $m)?;
            let result = left $o right;
            $m.mem_set(0, result).unwrap();
            return Ok(Signal::None)
        } 
    }
}

pub fn add(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    math!(+, v, m)
}

pub fn sub(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    math!(-, v, m)
}

pub fn mul(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    math!(*, v, m)
}

pub fn div(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    math!(/ , v, m)
}

#[cfg(test)]
mod test;
