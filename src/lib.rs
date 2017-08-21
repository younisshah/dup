extern crate libc;

use libc::c_int;
use std::fs::File;
use std::io;
use std::os::unix::io::FromRawFd;
use std::os::unix::io::AsRawFd;

/// Dup represents a RW Pipe
pub struct Dup;

impl Dup {
    /// `new` returns a tuple of Files representing a Read Write Pipe
    ///
    /// # Arguments
    ///
    /// * `fd` - A file descriptor
    ///
    /// # Example
    ///
    ///  // Proxy stdio, logging to your preferred logger and then re-write to the original stdio stream.
    ///
    /// `let mut rw = dup::Dup::new(1);
    ///
    ///     thread::spawn(|| {
    ///         println!("Hello World");
    ///     });
    ///
    /// let mut reader = BufReader::new(rw.0);
    /// let mut s = String::new();
    /// reader.read_to_string(&mut s);
    /// rw.1.write(s.as_bytes());`
    ///
    pub fn new(fd: c_int) -> (File, File) {
        let pipe = Pipe::new().unwrap();
        unsafe {
            let fd = libc::dup(fd);
            let writer: File = File::from_raw_fd(fd as c_int);
            libc::dup2(writer.as_raw_fd(), fd);
            (pipe.reader, writer)
        }
    }
}


// Copyright (C) 2015 Mickaël Salaün
// For License check https://github.com/stemjail/fd-rs/tree/master/src
mod raw {
    use libc::c_int;
    // From asm-generic/fcntl.h
    pub const O_CLOEXEC: c_int = 0o2000000;

    #[repr(C)]
    pub struct PipeFds {
        pub reader: c_int,
        pub writer: c_int,
    }

    extern {
        pub fn pipe2(fds: *mut PipeFds, flags: c_int) -> c_int;
    }
}

/// A thread-safe `pipe(2)` interface.
///
/// Create a reader and a writer `File` for each part of the pipe.
pub struct Pipe {
    pub reader: File,
    pub writer: File,
}

impl Pipe {
    pub fn new() -> io::Result<Pipe> {
        let mut fds = raw::PipeFds {
            reader: -1,
            writer: -1
        };
        if unsafe { raw::pipe2(&mut fds, raw::O_CLOEXEC) } != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(Pipe {
            reader: unsafe { File::from_raw_fd(fds.reader) },
            writer: unsafe { File::from_raw_fd(fds.writer) },
        })
    }
}