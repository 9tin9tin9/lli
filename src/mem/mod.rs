use ahash::AHashMap;
use super::error::Error;

pub struct Mem{
    pmem: Vec<f64>,
    nmem: Vec<f64>,
    var: AHashMap<String, isize>,
    label: AHashMap<String, usize>,
}

impl Mem{
    pub fn new() -> Mem {
        let mut m = Mem {
            pmem: Vec::from([0.0; 10000]),
            nmem: Vec::with_capacity(10000),
            var: AHashMap::new(),
            label: AHashMap::new(),
        };
        m.nmem.push(0.0);
        m
    }
    pub fn mem_at(&self,i: isize) -> Result<f64, Error>{
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
    pub fn var_add(&mut self, l: String, p: isize){
        self.var.insert(l, p);
    }
    pub fn var_find(&self, l: &str) -> Result<isize, Error>{
        match self.var.get(l) {
            Some(v) => Ok(*v),
            None => Err(Error::UnknownVarName(l.to_string()))
        }
    }

    pub fn read_ltl(&self, i: isize) -> Result<String, Error> {
        let mut v : Vec<u8> = Vec::new();
        let mut zero_count = 0;
        let mut i = i;
        let next = |i: &mut isize|
            idx_incr(i, 1);
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
            next(&mut i);
        }
    }
    pub fn label_add(&mut self, l: String, p: usize){
         self.label.insert(l, p);
    }
    pub fn label_find(&self, l: &str) -> Result<usize, Error>{
        match self.label.get(l) {
            Some(i) => Ok(*i),
            None => Err(Error::UnknownLabel(l.to_string())),
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
