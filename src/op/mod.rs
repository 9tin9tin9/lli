use ahash::AHashMap;
use crate::lex::*;
use crate::mem::Mem;
use crate::code::Code;
use crate::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum Signal{
    None,
    SetLbl(usize),
    Jmp(usize),
    Ret,
    Src(String),
}

impl Signal{
    pub fn respond(
        &self, 
        m: &mut Mem, 
        code: &mut Code, 
        op_idx_table: &AHashMap<&'static str, usize>
    ){
        match *self {
            Signal::None => (),
            Signal::Jmp(idx) => {
                // simulate calling functions, 
                // push line number before jump to stack.
                // stack used by ret
                m.jmp_stack_push(code.ptr());
                // set which line to execute next
                code.ptr_set(idx);
                return;
            },
            Signal::Ret => {
                if let Some(ln) = m.jmp_stack_pop() {
                    code.ptr_set(ln+1);
                    return;
                }
            },
            Signal::SetLbl(label) => {
                // Update line number for label. 
                // Set to current line + 1 to prevent re-updating
                // label each time jumping to this line and execute
                m.label_set(label, code.ptr()+1);
            },
            Signal::Src(ref s) => {
                crate::read_from_file(
                    s,
                    m,
                    code,
                    op_idx_table);
            }
        };
        code.ptr_incr();
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
mod extra;
mod r#extern;

macro_rules! add_entry {
    ( $h:ident, $v:ident, $c:ident, $o:ident ) => {
        // push function pointer
        $v.push($c::$o as OpFunc);
        // add (op, func ptr) entry to hash table
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

    add_entry!(h, v, flow, jmp);
    add_entry!(h, v, flow, lbl);
    add_entry!(h, v, flow, ret);

    add_entry!(h, v, sys, exit);
    add_entry!(h, v, sys, open);
    add_entry!(h, v, sys, close);
    add_entry!(h, v, sys, read);
    add_entry!(h, v, sys, write);

    add_entry!(h, v, r#extern, src);

    add_entry!(h, v, extra, print_num);
}

pub fn exec(func_vec: &[OpFunc], m: &mut Mem, c: &Code) -> Result<Signal, Error>{
    let v = c.curr().unwrap();
    if let Tok::Sym(ref hi) = v[0] {
        // lookup function pointer and execute
        func_vec[hi.idx](&v[1..], m)
    }else{
        // should not be executed 
        // because there is already a type check during preprocess
        // written just to let rustc to compile
        Err(Error::WrongTokTypeForOp(v[0].to_type_str()))
    }
}

#[cfg(test)]
mod test;
