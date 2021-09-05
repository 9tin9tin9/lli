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
pub enum Tok{
    Num(f64), 
    Idx(isize), 
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
            Tok::Idx(i) => format!("Idx({})", i),
            Tok::Var(s) => format!("Var({})", s.sym),
            Tok::Ltl(s) => format!("Ltl({})", s),
            Tok::Sym(s) => format!("Sym({})", s.sym),
            Tok::Eof => "Eof".to_owned(),
        }
    }
    fn from_u8(vec: &[u8]) -> Result<Tok, String> {
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
                    Err(e) => Err(e.to_string()),
                }
            },
            // Idx
            b'[' => 
                if vec[len-1] != b']' {
                    Err("Unterminated idx".to_string())
                }else {
                    let s = unsafe { 
                        std::str::from_utf8_unchecked(vec) 
                    };
                    match s[1..len-1].parse::<isize>() {
                        Ok(i) => Ok(Tok::Idx(i)),
                        Err(e) => Err(e.to_string()),
                    }
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
            Tok::Idx(i) =>
                m.mem_at(*i),
            Tok::Var(n) =>
                m.mem_at(m.var_find(&n)?),
            _ =>
                Err(Error::WrongArgType(
                        vec![Tok::NUM_STR, Tok::IDX_STR, Tok::VAR_STR], 
                        self.to_type_str())),
        }
    }

    pub fn get_uint(&self, m: &Mem) -> Result<u64, Error>{
        let float = self.get_value(m)?;
        if float != (float as usize) as f64 {
            return Err(Error::NegativeOrNonIntergerSize(float));
        }
        Ok(float as u64)
    }

    pub fn get_loc(&self, m: &mut Mem) -> Result<isize, Error> {
        match self {
            Tok::Idx(i) => Ok(*i),
            Tok::Var(n) =>
                m.var_find(n),
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

fn eat_token(it: &[u8], len: usize, delim: u8, unexpct: u8, msg: &str) 
    -> Result<(Tok, usize), String> 
{
    #[derive(PartialEq)]
    enum State{
        WAITING,
        STARTED(bool),  // bool for isStringLiteral
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
            }
            continue;
        }else if c == delim {
            match state {
                State::WAITING => 
                    return Err(format!("Empty token. {}", msg)),
                State::STARTED(false) | State::ENDED =>
                    break,
                State::STARTED(true) => (),
            }
        }else if c == unexpct {
            match state {
                State::STARTED(true) => (),
                _ =>
                    return Err(format!("Unexpected '{}'", unexpct as char)),
            }
        }else if state == State::ENDED {
            return Err("Found non-delimeter after symbol ends".to_string());
        }else if c == b'"'{
            match state {
                State::WAITING => 
                    state = State::STARTED(true),
                State::STARTED(true) =>
                    if !escaped {
                        state = State::ENDED;
                    },
                _ => 
                    return Err("double quote should either appear at the beginning or at the end of token".to_string()),
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
                _ => return Err("Unkown escape sequence".to_string()),
            }
        }
        escaped = false;
        current.push(c);
    }
    let tok = Tok::from_u8(&current);
    Ok((tok?, len))
}

fn eat_operator(slice: &[u8], len: usize) -> Result<(Tok, usize), String> {
    eat_token(slice, len, b':', b',', "Expects operator")
}

fn eat_args(slice: &[u8], len: usize) -> Result<(Tok, usize), String> {
    eat_token(slice, len, b',', b':', "Expects argument")
}

pub fn tokenize(line: &str) -> Result<Vec<Tok>, String>{
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
            return Err("Expects symbol as operator".to_string()),
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
