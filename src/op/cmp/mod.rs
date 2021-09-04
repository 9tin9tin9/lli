use crate::mem::Mem;
use crate::lex::Tok;
use super::*;

fn parse_arg(v: &[Tok], m: &Mem) -> Result<(f64, f64), Error>{
    argc_guard!(v, 2);
    Ok((v[0].get_value(m)?, 
        v[1].get_value(m)?))
}

macro_rules! cmp {
    ( $o:tt, $v:expr, $m:expr ) => {
        {
            let (left, right) = parse_arg($v, $m)?;
            let result = (left $o right) as i8 as f64;
            $m.mem_set(0, result).unwrap();
            return Ok(Signal::None)
        }
    }
}

pub fn eq(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    cmp!(==, v, m)
}

pub fn ne(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    cmp!(!=, v, m)
}

pub fn gt(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    cmp!(>, v, m)
}

pub fn lt(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    cmp!(<, v, m)
}

#[cfg(test)]
mod test;
