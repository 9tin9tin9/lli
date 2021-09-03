use ahash::AHashMap;
use crate::lex::*;
use crate::mem::Mem;
use crate::code::Code;
use crate::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum Signal{
    None,
    SetLbl(String),
    Jmp(usize),
    Ret,
    Skp,
}

impl Signal{
    pub fn respond(&self, m: &mut Mem, code: &mut Code) -> Result<(), Error>{
        match *self {
            Signal::None => (),
            Signal::Jmp(idx) => {
                m.jmp_stack_push(code.ptr());
                code.ptr_set(idx);
                return Ok(());
            },
            Signal::Ret => {
                if let Some(ln) = m.jmp_stack_pop() {
                    code.ptr_set(ln+1);
                    return Ok(());
                }
            }
            Signal::SetLbl(ref label) => {
                m.label_add(label.to_owned(), code.ptr()+1);
            },
            Signal::Skp => {
                code.ptr_incr();
            },
        };
        code.ptr_incr();
        Ok(())
    }
}

type OpFunc = fn(&[Tok], &mut Mem) -> Result<Signal, Error>;

#[macro_use]
macro_rules! argc_guard {
    ( $v:expr, $e:expr ) => {
        if $v.len() != $e {
            return Err(Error::WrongArgCount($e, $v.len()));
        }
    }
}

mod nop;
mod mem;
mod math;
mod cmp;
mod logic;
mod flow;
mod sys;

macro_rules! add_entry {
    ( $h:ident, $c:ident, $o:ident ) => {
        $h.insert(stringify!($o), $c::$o as OpFunc);
    };
}

lazy_static! {
    static ref OP_TABLE : AHashMap<
        &'static str, 
        OpFunc 
    > = {
        let mut h = AHashMap::new();
        add_entry!(h, nop, nop);

        add_entry!(h, mem, mov);
        add_entry!(h, mem, cpy);
        add_entry!(h, mem, var);
        add_entry!(h, mem, incr);
        add_entry!(h, mem, decr);
        add_entry!(h, mem, allc);

        add_entry!(h, math, add);
        add_entry!(h, math, sub);
        add_entry!(h, math, mul);
        add_entry!(h, math, div);
        h.insert("mod", math::r#mod as OpFunc);

        add_entry!(h, cmp, eq);
        add_entry!(h, cmp, ne);
        add_entry!(h, cmp, gt);
        add_entry!(h, cmp, lt);

        add_entry!(h, logic, and);
        add_entry!(h, logic, or);
        add_entry!(h, logic, not);

        add_entry!(h, flow, skp);
        add_entry!(h, flow, jmp);
        add_entry!(h, flow, lbl);
        add_entry!(h, flow, ret);

        add_entry!(h, sys, read);
        add_entry!(h, sys, write);
        h
    };
}

pub fn preload_label(m: &mut Mem, c: &Code) -> Result<(), Error>{
    let v = c.last().unwrap();
    if v.len() == 0 {
        return Ok(());
    }
    if let Tok::Sym(ref n) = v[0] {
        if n == "lbl" {
            m.label_add(
                v[1].get_sym()?
                    .to_owned(), 
                c.len());
        };
        Ok(())
    }else{
        Err(Error::WrongTokTypeForOp(v[0].to_type_str()))
    }
}

pub fn exec(m: &mut Mem, c: &Code) -> Result<Signal, Error>{
    let v = c.curr().unwrap();
    if v.len() == 0 {
        return nop::nop(v, m);
    }
    
    if let Tok::Sym(ref n) = v[0] {
        let name: &str = n;
        match OP_TABLE.get(name) {
            Some(f) =>
                f(&v[1..], m),
            None =>
                Err(Error::UnknownOp(name.to_string()))
        }
    }else{
        Err(Error::WrongTokTypeForOp(v[0].to_type_str()))
    }
}

#[cfg(test)]
mod test;
