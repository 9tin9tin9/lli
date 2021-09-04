// #[macro_use]
// extern crate lazy_static;
mod error;
mod lex;
mod code;
mod mem;
mod op;
use std::io::{self, BufRead};
use std::fs::File;
use std::env;
use ahash::AHashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return;
    }
    let file = File::open(args[1].clone())
                        .unwrap_or_else(|e|
                            panic!("{:?}", e));

    let mut m = mem::Mem::new();
    let mut code = code::Code::new();
    let mut op_idx_table: AHashMap<&'static str, usize> = AHashMap::new();
    let mut func_vec: Vec<op::OpFunc> = Vec::new();
    op::init_op_table(&mut op_idx_table, &mut func_vec);

    let lines = io::BufReader::new(file).lines();
    for line in lines {
        match line {
            Ok(s) => {
                if s.chars().all(|c| c.is_ascii_whitespace()){
                    continue;
                }
                let t = lex::tokenize(&s).unwrap();
                code.push(t);
                op::preprocess(&op_idx_table, &mut m, &mut code).unwrap()
            },
            Err(e) => panic!("{}", e),
        };
    }
    loop {
        if code.ptr() >= code.len() {
            break;
        }
        op::exec(&func_vec, &mut m, &code)
            .unwrap()
            .respond(&mut m, &mut code).unwrap();
    };
    // println!("{}", m.mem_at(0).unwrap());
}
