use core::fmt;

use super::ioctl::*;
use super::FileHandle;
use crate::net::Socket;
use crate::syscall::{SysError, SysResult};
use alloc::boxed::Box;
use rcore_fs::vfs::PollStatus;

// TODO: merge FileLike to FileHandle ?
// TODO: fix dup and remove Clone
#[derive(Clone)]
pub enum FileLike {
    File(FileHandle),
    Socket(Box<dyn Socket>),
}

impl FileLike {
    pub fn read(&mut self, buf: &mut [u8]) -> SysResult {
        let len = match self {
            FileLike::File(file) => file.read(buf)?,
            FileLike::Socket(socket) => socket.read(buf).0?,
        };
        Ok(len)
    }
    pub fn write(&mut self, buf: &[u8]) -> SysResult {
        let len = match self {
            FileLike::File(file) => file.write(buf)?,
            FileLike::Socket(socket) => socket.write(buf, None)?,
        };
        Ok(len)
    }
    pub fn ioctl(&mut self, request: usize, arg1: usize, arg2: usize, arg3: usize) -> SysResult {
        match request {
            // TODO: place flags & path in FileLike instead of FileHandle/Socket
            FIOCLEX => Ok(0),
            FIONBIO => Ok(0),
            _ => {
                match self {
                    FileLike::File(file) => file.io_control(request as u32, arg1)?,
                    FileLike::Socket(socket) => {
                        socket.ioctl(request, arg1, arg2, arg3)?;
                    }
                }
                Ok(0)
            }
        }
    }
    pub fn poll(&self) -> Result<PollStatus, SysError> {
        let status = match self {
            FileLike::File(file) => file.poll()?,
            FileLike::Socket(socket) => {
                let (read, write, error) = socket.poll();
                PollStatus { read, write, error }
            }
        };
        Ok(status)
    }

    pub fn fcntl(&mut self, cmd: usize, arg: usize) -> SysResult {
        match self {
            FileLike::File(file) => file.fcntl(cmd, arg)?,
            FileLike::Socket(socket) => {
                //TODO
            }
        }
        Ok(0)
    }
}

impl fmt::Debug for FileLike {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileLike::File(file) => write!(f, "File({:?})", file),
            FileLike::Socket(socket) => write!(f, "Socket({:?})", socket),
        }
    }
}
