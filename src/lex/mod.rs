use crate::error::Error;
use super::mem::Mem;

#[derive(Clone, PartialEq, Debug)]
pub struct HashIdx {
    pub sym: String,
    pub idx: usize,
}

impl HashIdx{
    pub fn new(s: &str, i: usize) -> Self{
        HashIdx {
            sym: s.to_owned(),
            idx: i,
        }
    }
    pub fn from_str(s: &str) -> Self {
        HashIdx {
            sym: s.to_owned(),
            idx: 0,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Idx {
    Num(isize),
    Idx(Box<Idx>),
    Var(HashIdx),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Tok{
    Num(f64), 
    Idx(Idx), 
    Var(HashIdx),
    Ltl(String),
    Sym(HashIdx),  // includes label and operator
    Eof,
}

impl Tok{
    pub const NUM_STR : &'static str = "Num";
    pub const IDX_STR : &'static str = "Idx";
    pub const VAR_STR : &'static str = "Var";
    pub const LTL_STR : &'static str = "Ltl";
    pub const SYM_STR : &'static str = "Sym";
    pub const EOF_STR : &'static str = "Eof";

    pub fn to_type_str(&self) -> &'static str{
        match *self {
            Tok::Num(_) => Tok::NUM_STR,
            Tok::Idx(_) => Tok::IDX_STR,
            Tok::Var(_) => Tok::VAR_STR,
            Tok::Ltl(_) => Tok::LTL_STR,
            Tok::Sym(_) => Tok::SYM_STR,
            Tok::Eof => Tok::EOF_STR,
        }
    }
     
    pub fn to_str(&self) -> String{
        match self {
            Tok::Num(n) => format!("Num({})", n),
            Tok::Idx(i) => format!("Idx({:?})", i),
            Tok::Var(s) => format!("Var({})", s.sym),
            Tok::Ltl(s) => format!("Ltl({})", s),
            Tok::Sym(s) => format!("Sym({})", s.sym),
            Tok::Eof => "Eof".to_owned(),
        }
    }

    fn eat_idx(vec: &[u8]) -> Result<Idx, Error> {
        let len = vec.len();
        if vec[len-1] != b']' {
            return Err(Error::UnterminatedIdx)
        } else if len == 2 {
            return Err(Error::EmptyIdx)
        }

        match vec[1] {
            // Var
            b'$' => {
                if len == 3 {
                    return Err(Error::MissingVarName)
                }
                let s = unsafe {
                    std::str::from_utf8_unchecked(&vec[2..len-1])
                };
                return Ok(Idx::Var(HashIdx::new(s, 0)))
            },
            // Idx
            b'[' => {
                return Ok(Idx::Idx(Box::new(Tok::eat_idx(&vec[1..len-1])?)));
            },
            // Num
            _ => {
                let s = unsafe { 
                    std::str::from_utf8_unchecked(&vec[1..len-1]) 
                };
                match s.parse::<isize>() {
                    Ok(i) => {
                        return Ok(Idx::Num(i));
                    },
                    Err(e) => Err(Error::ParseIdxError(e)),
                }
            },
        }
    }

    fn from_u8(vec: &[u8]) -> Result<Tok, Error> {
        let len = vec.len();
        if len == 0 {
            return Ok(Tok::Eof);
        }
        match vec[0] {
            // Num
            b'-' | b'0'..=b'9' => {
                let s = unsafe { 
                    std::str::from_utf8_unchecked(vec) 
                };
                match s.parse::<f64>() {
                    Ok(f) => Ok(Tok::Num(f)),
                    Err(e) => Err(Error::ParseNumError(e)),
                }
            },
            // Idx
            b'[' => {
                Ok(Tok::Idx(Tok::eat_idx(&vec)?))
            },
            // Var
            b'$' => {
                let s = unsafe { 
                    std::str::from_utf8_unchecked(&vec[1..]) 
                };
                Ok(Tok::Var(HashIdx::from_str(s)))
            },
            // Ltl
            b'"' => {
                let s = unsafe { 
                    std::str::from_utf8_unchecked(vec) 
                };
                Ok(Tok::Ltl(s[1..len-1].to_owned()))
            }
            // Sym
            _ => { 
                let s = unsafe { 
                    std::str::from_utf8_unchecked(vec) 
                };
                Ok(Tok::Sym(HashIdx::from_str(s)))
            }
        }
    }

    pub fn get_value(&self, m: &Mem) -> Result<f64, Error>{
        match self {
            Tok::Num(f) => Ok(*f),
            Tok::Idx(ref idx) => {
                let mut idx = idx;
                let mut layer: usize = 0;
                while let Idx::Idx(a) = idx {
                    idx = a;
                    layer += 1;
                }

                let mut d = match idx {
                    Idx::Num(n) => m.mem_at(*n)?,
                    Idx::Var(v) => m.mem_at(m.var_find(v)?)?,
                    Idx::Idx(_) => 0f64
                };

                for _ in 0..layer {
                    if d != d as isize as f64 {
                        return Err(Error::NotInterger(d));
                    }
                    d = m.mem_at(d as isize)?;
                }
                Ok(d)
            }
            Tok::Var(n) => m.mem_at(m.var_find(n)?),
            _ =>
                Err(Error::WrongArgType(
                        vec![Tok::NUM_STR, Tok::IDX_STR, Tok::VAR_STR], 
                        self.to_type_str())),
        }
    }

    pub fn get_uint(&self, m: &Mem) -> Result<u64, Error>{
        let float = self.get_value(m)?;
        if float != (float as u64) as f64 {
            return Err(Error::NegativeOrNotInterger(float));
        }
        Ok(float as u64)
    }

    pub fn get_int(&self, m: &Mem) -> Result<i64, Error>{
        let float = self.get_value(m)?;
        if float != (float as i64) as f64 {
            return Err(Error::NotInterger(float));
        }
        Ok(float as i64)
    }

    pub fn get_loc(&self, m: &mut Mem) -> Result<isize, Error> {
        match self {
            Tok::Idx(idx) => {
                let mut idx = idx;
                let mut layer: usize = 0;
                while let Idx::Idx(a) = idx {
                    idx = a;
                    layer += 1;
                }

                let mut l = match idx {
                    Idx::Num(n) => *n,
                    Idx::Var(v) => m.var_find(v)?,
                    Idx::Idx(_) => 0isize
                };

                for _ in 0..layer {
                    let mut d = l as f64;
                    d = m.mem_at(l)?;
                    if d != d as isize as f64 {
                        return Err(Error::NotInterger(d));
                    }
                    l = d as isize;
                }
                Ok(l)
            },
            Tok::Var(n) => m.var_find(n),
            Tok::Ltl(_) =>
                self.create_ltl(m),
            _ =>
                Err(Error::WrongArgType(
                        vec![Tok::IDX_STR, Tok::VAR_STR, Tok::LTL_STR], 
                        self.to_type_str())),
        }
    }

    pub fn write_value(&self, m: &mut Mem, v: f64) -> Result<(), Error> {
        let idx = self.get_loc(m)?;
        if idx < 0 {
            return Err(Error::WriteToNMem(idx));
        }
        m.mem_set(idx, v)
    }

    // alloc memory for String in nmem
    // Strings are terminated by two 0f64 consecutively
    pub fn create_ltl(&self, m: &mut Mem) -> Result<isize, Error> {
        if let Tok::Ltl(ref s) = self {
            let idx = m.nmem_len() as isize;
            // turn string to vec
            for c in s.as_bytes() {
                m.nmem_allc(&[*c as f64]);
            }
            // null for utf16
            m.nmem_allc(&[0f64; 2]);
            // change to negative
            Ok(-idx)
        }else{
            Err(Error::WrongArgType(
                    vec![Tok::LTL_STR], 
                    self.to_type_str()))
        }
    }

    pub fn get_sym<'a>(&'a self) -> Result<&'a HashIdx, Error> {
        if let Tok::Sym(ref s) = self {
            return Ok(s)
        }else{
            Err(Error::WrongArgType(
                    vec![Tok::SYM_STR],
                    self.to_type_str()))
        }
    }

}

fn eat_token(it: &[u8], len: usize, delim: u8, unexpct: u8)
    -> Result<(Tok, usize), Error> 
{
    #[derive(PartialEq)]
    enum State{
        WAITING,
        STARTED(bool),  // bool for isStringLiteral?
        ENDED,
    }
    let mut current = Vec::with_capacity(len);
    let mut escaped = false;
    let mut state = State::WAITING;
    let mut len = 0;
    for c in it{
        len += 1;
        let mut c = *c;
        if c == b'#' && state != State::STARTED(true) {
            break;
        }else if c == b' ' || c == b'\t' {
            if state == State::STARTED(false) {
                state = State::ENDED;
                continue;
                // break;
            }else if state != State::STARTED(true) {
                continue;
            }
        }else if c == delim {
            match state {
                State::WAITING => 
                    return Err(Error::EmptyToken),
                State::STARTED(false) | State::ENDED =>
                    break,
                State::STARTED(true) => (),
            }
        }else if c == unexpct {
            match state {
                State::STARTED(true) => (),
                _ =>
                    return Err(Error::UnexpectedChar(unexpct as char)),
            }
        }else if state == State::ENDED {
            return Err(Error::NonDelimAfterSymEnd(c as char));
        }else if c == b'"'{
            match state {
                State::WAITING => 
                    state = State::STARTED(true),
                State::STARTED(true) =>
                    if !escaped {
                        state = State::ENDED;
                    },
                _ => 
                    return Err(Error::DoubleQuoteInMiddle),
            }
        }else if c == b'\\' {
            if state == State::STARTED(true) {
                escaped = !escaped;
                if !escaped {
                    current.push(c);
                }
                continue;
            }
        }else if state == State::WAITING {
            state = State::STARTED(false);
        }else if escaped == true {
            c = match c {
                b'n' => b'\n',
                b't' => b'\t',
                _ => return Err(Error::UnknownEscapeSequence(c as char)),
            }
        }
        escaped = false;
        current.push(c);
    }
    let tok = Tok::from_u8(&current);
    Ok((tok?, len))
}

fn eat_operator(slice: &[u8], len: usize) -> Result<(Tok, usize), Error>  {
    eat_token(slice, len, b':', b',')
}

fn eat_args(slice: &[u8], len: usize) -> Result<(Tok, usize), Error>  {
    eat_token(slice, len, b',', b':')
}

pub fn tokenize(line: &str) -> Result<Vec<Tok>, Error>{
    let mut v : Vec<Tok> = Vec::with_capacity(5);
    let bytes = line.as_bytes();
    let len = line.len();
    let mut read_len = 0;
    // operator
    let (op, l) = eat_operator(bytes, len)?;
    read_len += l;
    v.push(match op {
        Tok::Sym(_) => 
            op,
        Tok::Eof =>
            return Ok(v),
        _ => 
            return Err(Error::WrongTokTypeForOp(op.to_type_str())),
    });
    // args
    loop {
        let (arg, l) = eat_args(&bytes[read_len..], len-read_len)?;
        v.push(match arg {
            Tok::Eof =>
                return Ok(v),
            _ =>
                arg,
        });
        read_len += l;
    }
}

#[cfg(test)]
mod test;
