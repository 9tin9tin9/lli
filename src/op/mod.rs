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

pub type OpFunc = fn(&[Tok], &mut Mem) -> Result<Signal, Error>;

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
    ( $h:ident, $v:ident, $c:ident, $o:ident ) => {
        $v.push($c::$o as OpFunc);
        $h.insert(stringify!($o), $v.len()-1);
    };
}

pub fn init_op_table(h: &mut AHashMap<&'static str, usize>, v: &mut Vec<OpFunc>){
    add_entry!(h, v, nop, nop);

    add_entry!(h, v, mem, mov);
    add_entry!(h, v, mem, cpy);
    add_entry!(h, v, mem, var);
    add_entry!(h, v, mem, incr);
    add_entry!(h, v, mem, decr);
    add_entry!(h, v, mem, allc);

    add_entry!(h, v, math, add);
    add_entry!(h, v, math, sub);
    add_entry!(h, v, math, mul);
    add_entry!(h, v, math, div);

    v.push(math::r#mod as OpFunc);
    h.insert("mod", v.len());

    add_entry!(h, v, cmp, eq);
    add_entry!(h, v, cmp, ne);
    add_entry!(h, v, cmp, gt);
    add_entry!(h, v, cmp, lt);

    add_entry!(h, v, logic, and);
    add_entry!(h, v, logic, or);
    add_entry!(h, v, logic, not);

    add_entry!(h, v, flow, skp);
    add_entry!(h, v, flow, jmp);
    add_entry!(h, v, flow, lbl);
    add_entry!(h, v, flow, ret);

    add_entry!(h, v, sys, read);
    add_entry!(h, v, sys, write);
    add_entry!(h, v, sys, print_num);
}

pub fn preprocess(
    op_idx_table: &AHashMap<&'static str, usize>, 
    m: &mut Mem, 
    c: &mut Code
    ) -> Result<(), Error>
{
    let v = c.last().unwrap();
    if v.len() == 0 {
        c.func_idx_push(0);
        return Ok(());
    }
    if let Tok::Sym(ref n) = v[0] {
        if n == "lbl" {
            m.label_add(
                v[1].get_sym()?
                    .to_owned(), 
                c.len());
        };
        let n: &str = &n.to_owned();
        c.func_idx_push(
            match op_idx_table.get(n) {
                Some(i) => *i,
                None => return Err(Error::UnknownOp(n.to_string())),
            });
        Ok(())
    }else{
        Err(Error::WrongTokTypeForOp(v[0].to_type_str()))
    }
}

pub fn exec(func_vec: &[OpFunc], m: &mut Mem, c: &Code) -> Result<Signal, Error>{
    let (idx, v) = c.curr().unwrap();
    if v.len() == 0 {
        return nop::nop(v, m);
    }
    func_vec[idx](&v[1..], m)
}

#[cfg(test)]
mod test;
