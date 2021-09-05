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
    argc_guard!(v, 1);
    let val = v[0].get_value(m)?;
    let mut f = unsafe { File::from_raw_fd(1) };
    f.write_fmt(format_args!("{}", val)).unwrap();
    f.into_raw_fd();
    Ok(Signal::None)
}
