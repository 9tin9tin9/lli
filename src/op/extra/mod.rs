use crate::error::Error;
use crate::lex::Tok;
use crate::mem::Mem;
use super::*;
use std::{
    fs::File,
    os::unix::io::{ FromRawFd, IntoRawFd },
    io::Write,
};

pub fn print_num(v: &[Tok], m: &mut Mem) -> Result<Signal, Error> {
    argc_guard!(v, 2);
    let fd = v[0].get_uint(m)? as i32;
    if !m.fd[fd as usize] {
        return Err(Error::BadFileDescriptor(fd));
    }
    let val = v[1].get_value(m)?;
    let mut f = unsafe { File::from_raw_fd(fd) };
    f.write_fmt(format_args!("{}", val)).unwrap();
    f.into_raw_fd();
    Ok(Signal::None)
}
