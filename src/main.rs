mod error;
mod lex;
mod code;
mod mem;
mod op;
use std::io::{self, BufRead};
use std::fs::File;
use std::env;
use ahash::AHashMap;
use mem::Mem;
use code::Code;
use lex::Tok;
use error::Error;
// used by tests
#[macro_use]
extern crate matches;

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
    match opcode {
        // lbl | alias
        21 | 22 => if let Tok::Sym(ref mut hi) = t[1] {
            hi.idx = match m.label_hash.get(&hi.sym) {
                Some(i) => *i,
                None => {
                    let idx = m.label_add(c.len());
                    m.label_hash.insert(hi.sym.to_owned(), idx);
                    idx
                },
            };
        },
        // var
        3 => if let Tok::Sym(ref mut hi) = t[1] {
            if let None = m.var_hash.get(&hi.sym) {
                hi.idx = m.var_add(0);
                m.var_hash.insert(hi.sym.to_owned(), hi.idx);
            }
        },
        _ => (),
    };
    Ok(())
}

fn replabe_lbl(tok: &mut Tok, m: &Mem)  -> Result<(), Error>{
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
            match hi.idx {
                // var | lbl
                3 | 21 => (),
                // jmp
                19 => replabe_lbl(&mut line[1], m)?,
                // jc | als
                20 | 22 => replabe_lbl(&mut line[2], m)?,

                _ => for a in &mut line[1..] {
                    // Var or VarIdx
                    if let Tok::Var(ref mut hi) | Tok::VarIdx(ref mut hi) = a {
                        hi.idx = match m.var_hash.get(&hi.sym) {
                            Some(i) => *i,
                            None => 
                                return Err(Error::UndefinedVar(hi.sym.to_owned())),
                        };
                    }
                },
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
                    .unwrap_or_else(|e|
                        panic!("{:?}", e));
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        match line {
            Ok(s) => {
                let t = lex::tokenize(&s).unwrap();
                // preprocess and push t to code
                preprocess(op_idx_table, m, code, t).unwrap();
            },
            Err(e) => panic!("{}", e),
        };
    }
    replace_sym(m, code)
}

fn main() -> Result<(), Error>{
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return Ok(());
    }
    let mut m = mem::Mem::new();
    let mut code = code::Code::new();
    let mut op_idx_table: AHashMap<&'static str, usize> = AHashMap::new();
    let mut op_vec: Vec<op::OpFunc> = Vec::new();
    op::init_op_table(&mut op_idx_table, &mut op_vec);
    read_from_file(&args[1], &mut m, &mut code, &mut op_idx_table).unwrap();
    while code.ptr() < code.len() {
        op::exec(&op_vec, &mut m, &code)
            .unwrap()
            .respond(&mut m, &mut code, &op_idx_table)?;
    };
    Ok(())
}
