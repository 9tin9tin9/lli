use ahash::AHashMap;
use super::error::Error;
use crate::lex::HashIdx;
use std::os::unix::io::{ FromRawFd, IntoRawFd };
use std::fs::File;

#[derive(Debug)]
pub struct Mem{
    pmem: Vec<f64>,
    nmem: Vec<f64>,
    pub var_hash: AHashMap<String, usize>,
    var: Vec<isize>,
    pub label_hash: AHashMap<String, usize>,
    label: Vec<usize>,
    pub fd: Vec<bool>,
}

impl Mem{
    pub fn new() -> Mem {
        let fd_limit = 1024;
        let mut m = Mem {
            pmem: vec![0.0; 10000],
            nmem: Vec::with_capacity(10000),
            var_hash: AHashMap::new(),
            var: Vec::with_capacity(100000),
            label_hash: AHashMap::new(),
            label: Vec::with_capacity(100000),
            fd: vec![false; fd_limit],
        };
        m.nmem.push(0.0);
        unsafe {
            File::from_raw_fd(0).into_raw_fd();
            m.fd[0] = true;
            File::from_raw_fd(1).into_raw_fd();
            m.fd[1] = true;
            File::from_raw_fd(2).into_raw_fd();
            m.fd[2] = true;
        }
        m
    }
    pub fn mem_at(&self, i: isize) -> Result<f64, Error>{
        if i < 0 {
            self.nmem_at(-i as usize)
        }else{
            self.pmem_at(i as usize)
        }
    }
    pub fn mem_set(&mut self, i: isize, v: f64) -> Result<(), Error>{
        if i < 0 {
            self.nmem_set(-i as usize, v)
        }else{
            self.pmem_set(i as usize, v)
        }
    }
    pub fn pmem_at(&self, i: usize) -> Result<f64, Error>{
        if i >= self.pmem.len() {
            Err(Error::InvalidMemAccess(-(i as isize)))
        }else{
            Ok(self.pmem[i])
        }
    }
    pub fn pmem_set(&mut self, i: usize, v: f64) -> Result<(), Error>{
        if i >= self.pmem.len() {
            Err(Error::InvalidMemAccess(-(i as isize)))
        }else{
            Ok(self.pmem[i] = v)
        }
    }
    pub fn pmem_len(&self) -> usize{
        self.pmem.len()
    }
    pub fn pmem_allc(&mut self, v: &[f64]) {
        self.pmem.extend_from_slice(v);
    }
    pub fn nmem_at(&self, i: usize) -> Result<f64, Error>{
        if i >= self.nmem.len() {
            Err(Error::InvalidMemAccess(-(i as isize)))
        }else{
            Ok(self.nmem[i])
        }
    }
    pub fn nmem_set(&mut self, i: usize, v: f64) -> Result<(), Error>{
        if i >= self.nmem.len() {
            Err(Error::InvalidMemAccess(-(i as isize)))
        }else{
            Ok(self.nmem[i] = v)
        }
    }
    pub fn nmem_len(&self) -> usize{
        self.nmem.len()
    }
    pub fn nmem_allc(&mut self, v: &[f64]) {
        self.nmem.extend_from_slice(v);
    }
    pub fn var_add(&mut self, i: isize) -> usize {
        self.var.push(i);
        self.var.len()-1
    }
    pub fn var_set(&mut self, var: usize, idx: isize){
        self.var[var] = idx;
    }
    pub fn var_find(&self, hi: &HashIdx) -> Result<isize, Error>{
        match self.var.get(hi.idx) {
            Some(v) => Ok(*v),
            None => Err(Error::UndefinedVar(hi.sym.to_string()))
        }
    }

    pub fn read_ltl(&self, i: isize) -> Result<String, Error> {
        let mut v : Vec<u8> = Vec::new();
        let mut zero_count = 0;
        let mut i = i;
        loop {
            let c = self.mem_at(i)?;
            if c == 0.0 {
                if zero_count == 1{
                    v.pop();
                    return Ok(String::from_utf8(v).unwrap());
                }
                zero_count += 1;
            }else{
                zero_count = 0;
            }
            v.push(c as u8);
            idx_incr(&mut i, 1);
        }
    }
    pub fn label_add(&mut self, line: usize) -> usize{
         self.label.push(line);
         self.label.len()-1
    }
    pub fn label_set(&mut self, lbl: usize, line: usize){
        self.label[lbl] = line;
    }
    pub fn label_find(&self, hi: &HashIdx) -> Result<usize, Error>{
        match self.label.get(hi.idx) {
            Some(i) => Ok(*i),
            None => Err(Error::UnknownLabel(hi.sym.to_string())),
        }
    }
}

pub fn idx_incr(i: &mut isize, delta: isize){
    if *i < 0 {
        *i -= delta;
    }else {
        *i += delta;
    }
}

pub fn idx_decr(i: &mut isize, delta: isize){
    if *i < 0 {
        *i += delta;
    }else {
        *i -= delta;
    }
}

#[cfg(test)]
mod test;
