use crate::error::Error;
use crate::lex::Tok;
use crate::mem::{Mem, idx_incr};
use std::io::{ Write, Read };
use std::fs::File;
use std::os::unix::io::{ FromRawFd, IntoRawFd };
use super::*;

const MAX_INPUT: usize = 1024;

pub fn write(v: &[Tok], m: &mut Mem) -> Result<Signal, Error> {
    argc_guard!(v, 3);
    let fd = v[0].get_uint(m)? as i32;
    if !m.fd.contains(&fd) {
        return Err(Error::BadFileDescriptor(fd));
    }
    let mut f = unsafe { File::from_raw_fd(fd) };
    let mut src_idx = v[1].get_loc(m)?;
    let size = v[2].get_uint(m)?;
    let mut bytes_wrote = 0;
    for _ in 0..size as usize {
        f.write(&[m.mem_at(src_idx)? as u8]).unwrap();
        idx_incr(&mut src_idx, 1);
        bytes_wrote += 1;
    }
    // return file ownership to file descriptor, don't close the file here
    f.into_raw_fd();
    m.mem_set(0, bytes_wrote as f64)?;
    Ok(Signal::None)
}

pub fn read(v: &[Tok], m: &mut Mem) -> Result<Signal, Error>{
    argc_guard!(v, 3);
    let fd = v[0].get_uint(m)? as i32;
    if !m.fd.contains(&fd) {
        return Err(Error::BadFileDescriptor(fd));
    }
    let mut f = unsafe { File::from_raw_fd(fd) };
    let des_idx = v[1].get_loc(m)?;
    let size = v[2].get_uint(m)?;
    let size = size as usize;
    let mut buf = [0; MAX_INPUT];
    f.read(&mut buf).unwrap();
    f.into_raw_fd();
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
