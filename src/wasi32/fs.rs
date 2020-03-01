use super::*;
use crate::MemoryRef;

pub unsafe fn fd_write(
    ctx: &mut WasiCtx,
    memory: &MemoryRef,
    fd: Fd,
    iovs_ptr: Pointer,
    iovs_len: Size,
    nwritten: Pointer,
) -> Result<(), ()> {
    dbg!("fd~~~~~~~~~");
    let iovs = memory.slice(iovs_ptr as usize, iovs_len as usize);
    dbg!(&iovs);
    /*
    let iovs = dec_ciovec_slice(memory, iovs_ptr, iovs_len)?;
    let iovs: Vec<io::IoSlice> = iovs.iter().map(|vec| host::ciovec_to_host(vec)).collect();
    dbg!(&wasi_ctx.args);
    // perform unbuffered writes
    let entry = wasi_ctx.get_fd_entry_mut(fd)?;
    let isatty = entry.isatty();
    let desc = entry.as_descriptor_mut(wasi::__WASI_RIGHTS_FD_WRITE, 0)?;

    dbg!(&desc, &iovs, isatty);
    let host_nwritten = match desc {
        Descriptor::OsHandle(file) => {
            if isatty {
                SandboxedTTYWriter::new(file.deref_mut()).write_vectored(&iovs)?
            } else {
                file.write_vectored(&iovs)?
            }
        }
        Descriptor::Stdin => return Err(Error::EBADF),
        Descriptor::Stdout => {
            // lock for the duration of the scope
            let stdout = io::stdout();
            let mut stdout = stdout.lock();
            let nwritten = if isatty {
                SandboxedTTYWriter::new(&mut stdout).write_vectored(&iovs)?
            } else {
                stdout.write_vectored(&iovs)?
            };
            stdout.flush()?;
            nwritten
        }
        // Always sanitize stderr, even if it's not directly connected to a tty,
        // because stderr is meant for diagnostics rather than binary output,
        // and may be redirected to a file which could end up being displayed
        // on a tty later.
        Descriptor::Stderr => SandboxedTTYWriter::new(&mut io::stderr()).write_vectored(&iovs)?,
    };

    println!("     | *nwritten={:?}", host_nwritten);

    enc_usize_byref(memory, nwritten, host_nwritten)
    */
    Ok(())
}
