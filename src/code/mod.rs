use crate::lex::*;

pub struct Code{
    code: Vec<Vec<Tok>>,
    ptr: usize,
}

impl Code{
    pub fn new() -> Code{
        Code {
            code: Vec::with_capacity(10000),
            ptr: 0,
        }
    }
    pub fn push(&mut self, c: Vec<Tok>) -> usize{
        if self.code.len() + c.len() > self.code.capacity(){
            self.code.reserve(10000);
        }
        self.code.push(c);
        self.code.len()
    }
    pub fn at(&self, i: usize) -> Option<&[Tok]>{
        if i >= self.code.len() {
            None
        }else{
            Some(&self.code[i])
        }
    }
    pub fn last(&self) -> Option<&[Tok]>{
        if self.code.len() == 0 {
            None
        }else{
            Some(&self.code[self.code.len()-1])
        }
    }
    pub fn curr(&self) -> Option<&[Tok]>{
        self.at(self.ptr)
    }
    pub fn len(&self) -> usize{
        self.code.len()
    }
    pub fn ptr(&self) -> usize{
        self.ptr
    }
    pub fn ptr_set(&mut self, l: usize){
        self.ptr = l;
    }
    pub fn ptr_incr(&mut self){
        self.ptr += 1;
    }
}
