use crate::error::Error;
use crate::lex::Tok;
use crate::mem::{Mem, idx_incr};
use std::io::{ Write, Read };
use std::fs::{ File, OpenOptions };
use std::os::unix::io::{ FromRawFd, IntoRawFd };
use super::*;

const MAX_INPUT: usize = 1024;

//      exit: exit_code(Value)
pub fn exit(v: &[Tok], m: &mut Mem) -> Result<Signal, Error> {
    argc_guard!(v, 1);
    let exit_code = v[0].get_value(m)?;
    std::process::exit(exit_code as i32);
}

// Writes to file descriptor. No mutex. 
// [0] set to bytes slots written to fd
//      write: fd(Value), ptr(Ptr), size(Value)
pub fn write(v: &[Tok], m: &mut Mem) -> Result<Signal, Error> {
    argc_guard!(v, 3);
    let fd = v[0].get_uint(m)? as i32;
    // check if fd is opened
    if !m.fd[fd as usize] {
        return Err(Error::BadFileDescriptor(fd));
    }
    // create file from fd
    let mut f = unsafe { File::from_raw_fd(fd) };
    let mut src_idx = v[1].get_loc(m)?;
    let size = v[2].get_uint(m)?;
    // read from mem and write to file
    for _ in 0..size as usize {
        if let Err(e) = f.write(&[m.mem_at(src_idx)? as u8]){
            return Err(Error::IoError(e));
        }
        idx_incr(&mut src_idx, 1);
    }
    // return file ownership to file descriptor, 
    // don't close the file here
    f.into_raw_fd();
    m.mem_set(0, size as f64)?;
    Ok(Signal::None)
}

// Read from fd. No mutex
// [0] set to bytes slots read from fd
//      read: fd(Value), ptr(WPtr), size(Value)
pub fn read(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 3);
    let fd = v[0].get_uint(m)? as i32;
    // check if fd is opened
    if !m.fd[fd as usize] {
        return Err(Error::BadFileDescriptor(fd));
    }
    // create file from fd
    let mut f = unsafe { File::from_raw_fd(fd) };
    let des_idx = v[1].get_loc(m)?;
    let size = v[2].get_uint(m)?;
    let size = size as usize;
    let mut buf = [0; MAX_INPUT];
    // read from file
    if let Err(e) = f.read(&mut buf) {
        return Err(Error::IoError(e));
    }
    // transfer file ownership to fd prevent auto closing file
    f.into_raw_fd();
    // wrtie to mem
    for i in 0..MAX_INPUT {
        let c = buf[i] as f64;
        if c == 0.0 || i == size {
            m.mem_set(des_idx + i as isize, 0.0)?;
            m.mem_set(0, i as f64)?;
            break;
        }
        m.mem_set(des_idx + i as isize, c)?;
    }
    Ok(Signal::None)
}

// parse digits(boolean value) from right to left
fn parse_open_options(mut o_val: u64) -> Result<OpenOptions, Error> {
    let mut options = [false; 6];
    for o in &mut options {
        *o = match o_val % 10 {
            0 => false,
            1 => true,
            _ => return Err(Error::InvalidOpenOption(o_val)),
        };
        o_val /= 10;
    }
    Ok(OpenOptions::new()
        .read(options[0])
        .write(options[1])
        .append(options[2])
        .truncate(options[3])
        .create(options[4])
        .create_new(options[5])
        .clone())
}

// Open file and set [0] to fd
//      open: name(Ptr | Sym), option(Value)
//
// Open options: number consisting 6 or less digits
//
//  _ _ _ _ _ _
//  6 5 4 3 2 1
//
//  1) read
//  2) write
//  3) append
//  4) truncate
//  5) create
//  6) create_new
//
//  All digits should be either be 0 or 1, representing boolean value.
//  Boolean values will be passed to std::fs::OpenOptions.
//  Read rust docs for more details about each option.
//
//  Example: opening text.txt in read only mode
//      open:"text.txt",1
//
//  Example: opening text.txt in write-only mode, 
//           create file if it does not exists,
//           and will truncate it if it does.
//      open:"text.txt",11010
//
pub fn open(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 2);
    let name = if let Tok::Sym(idx) = &v[0] {
        idx.sym.clone()
    }else {
        let name_ptr = v[0].get_loc(m)?;
        m.read_ltl(name_ptr)?
    };
    let f = {
        let option = parse_open_options(v[1].get_uint(m)?)?;
        match option.open(name) {
            Ok(f) => f,
            Err(e) => return Err(Error::IoError(e)),
        }
    };
    let fd = f.into_raw_fd();
    m.fd[fd as usize] = true;
    m.mem_set(0, fd as f64)?;
    Ok(Signal::None)
}

// Close fd
//      close: fd(Value)
pub fn close(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 1);
    let fd = v[0].get_uint(m)? as i32;
    // check if fd is opened
    if !m.fd[fd as usize] {
        return Err(Error::BadFileDescriptor(fd));
    }
    // allow closing file automatically though drop
    unsafe { File::from_raw_fd(fd) };
    // mark fd as closed
    m.fd[fd as usize] = false;
    Ok(Signal::None)
}
