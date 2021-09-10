use crate::error::Error;
use crate::lex::Tok;
use crate::mem::Mem;
use super::*;
use std::{
    fs::File,
    os::unix::io::{ FromRawFd, IntoRawFd },
    io::Write,
};

// Write formatted value to fd
//      print_num: fd(Value, val(Value)
pub fn print_num(v: &[Tok], m: &mut Mem) -> Result<Signal, Error> {
    argc_guard!(v, 2);
    let fd = v[0].get_uint(m)? as i32;
    // check if fd is opened
    if !m.fd[fd as usize] {
        return Err(Error::BadFileDescriptor(fd));
    }
    let val = v[1].get_value(m)?;
    // open file
    let mut f = unsafe { File::from_raw_fd(fd) };
    // fmt float to string and write to fd
    f.write_fmt(format_args!("{}", val)).unwrap();
    // return file ownership to fd
    f.into_raw_fd();
    Ok(Signal::None)
}
