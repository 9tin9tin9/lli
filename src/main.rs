mod error;
mod lex;
mod code;
mod mem;
mod op;
use num_traits::FromPrimitive;
use std::io::{self, BufRead};
use std::fs::File;
use std::env;
use ahash::AHashMap;
use mem::Mem;
use code::Code;
use lex::Tok;
use error::Error;
#[macro_use]
extern crate num_derive;
// used by tests
#[macro_use]
extern crate matches;

static ERROR_MSG_LEVEL: usize = 1;

fn assign_opcode(
    op_idx_table: &AHashMap<&'static str, usize>, 
    c: &mut Code,
    t: &mut [Tok]
) -> Result<usize, Error> 
{
    if let Tok::Sym(ref mut n) = t[0] {
        // lookup and assign opcode
        let s: &str = &n.sym;
        n.idx = c.func_idx_push(
            match op_idx_table.get(s) {
                Some(i) => *i,
                None => return Err(Error::UnknownOp(s.to_string())),
            });
        Ok(n.idx)
    }else{
        return Err(Error::WrongTokTypeForOp(t[0].to_type_str()))
    }
}

fn create_symbol_table(
    opcode: usize,
    m: &mut Mem, 
    c: &mut Code, 
    t: &mut Vec<Tok>
) -> Result<(), Error>
{
    match FromPrimitive::from_usize(opcode).unwrap() {
        op::Opcode::Lbl | op::Opcode::Als => if let Tok::Sym(ref mut hi) = t[1] {
            hi.idx = match m.label_hash.get(&hi.sym) {
                Some(i) => *i,
                None => {
                    let idx = m.label_add(c.len()+1);
                    m.label_hash.insert(hi.sym.to_owned(), idx);
                    idx
                },
            };
        },
        op::Opcode::Var => if let Tok::Sym(ref mut hi) = t[1] {
            hi.idx = match m.var_hash.get(&hi.sym) {
                Some(i) => *i,
                None => {
                    let idx = m.var_add(0);
                    m.var_hash.insert(hi.sym.to_owned(), idx);
                    idx
                }
            }
        },
        _ => (),
    };
    Ok(())
}

fn replace_lbl(tok: &mut Tok, m: &Mem)  -> Result<(), Error>{
    if let Tok::Sym(ref mut hi) = tok {
        hi.idx = match m.label_hash.get(&hi.sym) {
            Some(i) => *i,
            None =>
                return Err(Error::UnknownLabel(hi.sym.clone())),
        }
    }
    Ok(())
}

// loop through all lines to replace symbols
fn replace_sym(m: &Mem, c: &mut Code) -> Result<(), Error> {
    for i in 0..c.len() {
        let line = c.at_mut(i).unwrap();
        if let Tok::Sym(ref hi) = line[0] {
            match FromPrimitive::from_usize(hi.idx).unwrap() {
                op::Opcode::Jmp => replace_lbl(&mut line[1], m)?,
                op::Opcode::Jc | op::Opcode::Als => replace_lbl(&mut line[2], m)?,
                _ => ()
            }
            for a in &mut line[1..] {
                // Var or VarIdx
                if let Tok::Var(ref mut hi) = a {
                    hi.idx = match m.var_hash.get(&hi.sym) {
                        Some(i) => *i,
                        None => 
                            return Err(Error::UndefinedVar(hi.sym.to_owned())),
                    };
                    continue;
                }else if let Tok::Idx(ref mut i) = a {
                    let mut idx = i;
                    while let lex::Idx::Idx(b) = idx {
                        idx = b;
                    }
                    if let lex::Idx::Var(v) = idx {
                        v.idx = match m.var_hash.get(&v.sym) {
                            Some(c) => *c,
                            None =>
                                return Err(Error::UndefinedVar(v.sym.to_owned())),
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn preprocess(
    op_idx_table: &AHashMap<&'static str, usize>, 
    m: &mut Mem, 
    c: &mut Code,
    mut t: Vec<Tok>
) -> Result<(), Error> {
    // skip empty lines
    if t.len() == 0 {
        return Ok(());
    }
    let opcode = assign_opcode(op_idx_table, c, &mut t)?;
    // create symbol table
    create_symbol_table(opcode, m, c, &mut t)?;
    c.push(t);
    Ok(())
}

fn read_from_file(
    file_name: &str, 
    m: &mut Mem, 
    code: &mut Code, 
    op_idx_table: &AHashMap<&'static str, usize>, 
) -> Result<(), Error> {
    let file = File::open(file_name)
                    .unwrap_or_else(|e| {
                        eprintln!("{}", e);
                        std::process::exit(1);
                    });
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        match line {
            Ok(s) => {
                let t = match lex::tokenize(&s){
                    Ok(t) => t,
                    Err(e) => { 
                        e.print(ERROR_MSG_LEVEL);
                        std::process::exit(1);
                    },
                };
                // preprocess and push t to code
                preprocess(op_idx_table, m, code, t).unwrap();
            },
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            },
        };
    }
    replace_sym(m, code)
}

fn run(
    m: &mut Mem, 
    code: &mut Code, 
    op_idx_table: &AHashMap<&'static str, usize>, 
    op_vec: &[op::OpFunc]
) -> Result<(), Error>
{
    while code.ptr() < code.len() {
        // the 2 unwrap_or_else closures have different return value
        op::exec(op_vec, m, code)?.respond(m, code, op_idx_table, op_vec)?;
    };
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return;
    }
    let mut m = mem::Mem::new();
    let mut code = code::Code::new();
    let mut op_idx_table: AHashMap<&'static str, usize> = AHashMap::new();
    let mut op_vec: Vec<op::OpFunc> = Vec::new();

    op::init_op_table(&mut op_idx_table, &mut op_vec);
    read_from_file(&args[1], &mut m, &mut code, &mut op_idx_table)
        .unwrap_or_else(|e| {
            e.print(ERROR_MSG_LEVEL);
            std::process::exit(1);
    });
    run(&mut m, &mut code, &op_idx_table, &op_vec)
        .unwrap_or_else(|e| {
            e.print(ERROR_MSG_LEVEL);
            std::process::exit(1);
    });
}
