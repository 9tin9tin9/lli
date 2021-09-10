use crate::mem::Mem;
use crate::lex::Tok;
use super::*;

enum Type{
    And, Or
}

fn unary_logic(t: Type, v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 2);
    let left = v[0].get_value(m)?;
    let right = v[1].get_value(m)?;
    // compare if non zero
    let result = match t {
        Type::And => 
            (left != 0.0 && right != 0.0) as i64 as f64,
        Type::Or => 
            (left != 0.0 || right != 0.0) as i64 as f64,
    };
    m.mem_set(0, result).unwrap();
    Ok(Signal::None)
}

pub fn and(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    unary_logic(Type::And, v, m)
}

pub fn or(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    unary_logic(Type::Or, v, m)
}

pub fn not(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 1);
    let value = v[0].get_value(m)?;
    // compare if non zero
    let result = (value == 0.0) as i64 as f64;
    m.mem_set(0, result).unwrap();
    Ok(Signal::None)
}

#[cfg(test)]
mod test;
