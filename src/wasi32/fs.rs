use num::PrimInt;
use std::convert::TryFrom;
use std::mem::{align_of, size_of};
use std::{io, ptr, slice};

use super::*;
use crate::{MemoryRef, YawError};

pub(crate) fn dec_usize(size: Size) -> usize {
    usize::try_from(size).unwrap()
}

#[derive(Debug)]
pub struct __wasi_ciovec_t {
    buf: *const u8,
    buf_len: Size,
}

fn dec_ptr(memory: &MemoryRef, ptr: Pointer, len: usize) -> Result<*const u8, ()> {
    // check for overflow
    let checked_len = (ptr as usize).checked_add(len as usize).unwrap(); // .ok_or(Error::EFAULT)?;

    // translate the pointer
    dbg!(ptr, checked_len);
    Ok(memory.slice(ptr as usize, checked_len).as_ptr())
    // let a = &a[..];
    // dbg!(a[0]);
    // let b = unsafe { slice::from_raw_parts(a.as_ptr(), 1) };
    // dbg!(a.as_ptr());
    // .ok_or(YawError::RuntimeError {}) /* Error::EFAULT */
    // .map(|mem| mem.as_ptr())
}

fn check_slice_of<T>(ptr: Pointer, len: Size) -> Result<(usize, usize), ()> {
    // check alignment, and that length doesn't overflow
    // if ptr as usize % align_of::<T>()!= 0 {
    //     return Err(Error::EINVAL);
    // }
    let len = dec_usize(len);
    let len_bytes = if let Some(len) = size_of::<T>().checked_mul(len) {
        len
    } else {
        return Err(() /*Error::EOVERFLOW*/);
    };

    Ok((len, len_bytes))
}

fn dec_raw_slice_of<'memory, T>(
    memory: &MemoryRef,
    ptr: Pointer,
    len: Size,
) -> Result<&'memory [T], ()> {
    dbg!(ptr, len);
    let (len, len_bytes) = check_slice_of::<T>(ptr, len)?;
    dbg!(len, len_bytes);
    let ptr = dec_ptr(memory, ptr, len_bytes)? as *const T;
    // dbg!(&buf, len);
    Ok(unsafe { slice::from_raw_parts(ptr, len) })
    // let b = unsafe { std::mem::transmute::<&[u8], *const T>(&buf) };
    // Ok(b)
}

pub(crate) fn decode_ciovec(
    memory: &MemoryRef,
    ptr: Pointer,
    len: Size,
) -> Result<Vec<__wasi_ciovec_t>, ()> {
    let raw_slice = dec_raw_slice_of::<(Size, Size)>(memory, ptr, len)?;
    dbg!(&raw_slice);
    raw_slice
        .iter()
        .map(|raw_iov| {
            dbg!(raw_iov);
            let len = dec_usize(PrimInt::from_le(raw_iov.1));
            let buf = PrimInt::from_le(raw_iov.0);
            Ok(__wasi_ciovec_t {
                buf: dec_ptr(memory, buf, len)? as *const u8,
                buf_len: len as Size,
            })
        })
        .collect()
}

pub(crate) unsafe fn ciovec_to_host(ciovec: &__wasi_ciovec_t) -> std::io::IoSlice {
    let slice = slice::from_raw_parts(ciovec.buf as *const u8, ciovec.buf_len as usize);
    std::io::IoSlice::new(slice)
}

pub unsafe fn fd_write(
    ctx: &mut WasiCtx,
    memory: &MemoryRef,
    fd: Fd,
    iovs_ptr: Pointer,
    iovs_len: Size,
    nwritten: Pointer,
) -> Result<(), ()> {
    dbg!("fd~~~~~~~~~");
    let iovs = decode_ciovec(memory, iovs_ptr, iovs_len)?;
    // let iovs = memory.slice(iovs_ptr as usize, iovs_len as usize);
    let iovs: Vec<io::IoSlice> = iovs.iter().map(|vec| ciovec_to_host(vec)).collect();
    dbg!(&iovs);

    // dbg!(&wasi_ctx.args);
    // // perform unbuffered writes
    let entry = ctx.get_fd_entry_mut(fd)?;
    // let isatty = entry.isatty();
    // let desc = entry.as_descriptor_mut(wasi::__WASI_RIGHTS_FD_WRITE, 0)?;
    //
    // dbg!(&desc, &iovs, isatty);
    // let host_nwritten = match desc {
    //     Descriptor::OsHandle(file) => {
    //         if isatty {
    //             SandboxedTTYWriter::new(file.deref_mut()).write_vectored(&iovs)?
    //         } else {
    //             file.write_vectored(&iovs)?
    //         }
    //     }
    //     Descriptor::Stdin => return Err(Error::EBADF),
    //     Descriptor::Stdout => {
    //         // lock for the duration of the scope
    //         let stdout = io::stdout();
    //         let mut stdout = stdout.lock();
    //         let nwritten = if isatty {
    //             SandboxedTTYWriter::new(&mut stdout).write_vectored(&iovs)?
    //         } else {
    //             stdout.write_vectored(&iovs)?
    //         };
    //         stdout.flush()?;
    //         nwritten
    //     }
    //     // Always sanitize stderr, even if it's not directly connected to a tty,
    //     // because stderr is meant for diagnostics rather than binary output,
    //     // and may be redirected to a file which could end up being displayed
    //     // on a tty later.
    //     Descriptor::Stderr => SandboxedTTYWriter::new(&mut io::stderr()).write_vectored(&iovs)?,
    // };
    //
    // println!("     | *nwritten={:?}", host_nwritten);
    //
    // enc_usize_byref(memory, nwritten, host_nwritten)

    Ok(())
}
