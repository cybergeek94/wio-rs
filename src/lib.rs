// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>

#![feature(io, old_io, os, std_misc)]

extern crate "winapi" as w;
extern crate "kernel32-sys" as k32;

pub mod file;

use std::error::FromError;
use std::ffi::OsString;
use std::fmt::{self, Display, Formatter};
use std::ptr::{null, null_mut};
use std::os::windows::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Error {
    code: w::DWORD,
}
impl Error {
    fn last() -> Error {
        Error { code: unsafe { k32::GetLastError() } }
    }
}
impl FromError<Error> for std::old_io::IoError {
    fn from_error(err: Error) -> std::old_io::IoError {
        std::old_io::IoError::from_errno(err.code as i32, true)
    }
}
impl FromError<Error> for std::io::Error {
    fn from_error(err: Error) -> std::io::Error {
        std::io::Error::from_os_error(err.code as i32)
    }
}
impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        let mut buf = [0; 0x1000];
        let len = unsafe {
            k32::FormatMessageW(w::FORMAT_MESSAGE_FROM_SYSTEM, null(), self.code, 0,
                                buf.as_mut_ptr(), buf.len() as w::DWORD, null_mut())
        };
        if len == 0 { return Err(fmt::Error) }
        let s = OsString::from_wide(&buf[..len as usize]);
        fmt.pad(&s.to_string_lossy())
    }
}

/// Sleep for the given time, waking if an I/O callback occurs.
///
/// Returns `false` if the timeout elapsed, or `true` if a callback occurred.
pub fn sleep(millis: u32) -> bool {
    unsafe { k32::SleepEx(millis, 1) != 0 }
}

#[cfg(test)]
mod test {
    use Error;
    use std::error::FromError;
    #[test]
    fn test_error() {
        println!("{}", Error::last());
        println!("{}", Error { code: 0 });
        println!("{}", <::std::old_io::IoError>::from_error(Error::last()));
        println!("{}", <::std::io::Error>::from_error(Error::last()));
    }
}
