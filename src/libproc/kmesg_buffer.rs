extern crate errno;
extern crate libc;

#[cfg(target_os = "macos")]
use std::fmt;
#[cfg(target_os = "macos")]
use std::{mem, ptr};

#[cfg(target_os = "macos")]
use crate::libproc::helpers;

#[cfg(target_os = "macos")]
use self::libc::c_int;

#[cfg(target_os = "linux")]
use std::fs::File;
#[cfg(target_os = "linux")]
use std::io::{BufRead, BufReader};
#[cfg(target_os = "linux")]
use std::sync::mpsc;
#[cfg(target_os = "linux")]
use std::sync::mpsc::Receiver;
#[cfg(target_os = "linux")]
use std::{thread, time};


// See https://opensource.apple.com/source/xnu/xnu-1456.1.26/bsd/sys/msgbuf.h
#[cfg(target_os = "macos")]
const MAX_MSG_BSIZE: c_int = 1024 * 1024;
#[cfg(target_os = "macos")]
const MSG_MAGIC: c_int = 0x063_061;

// See /usr/include/sys/msgbuf.h on your Mac.
#[cfg(target_os = "macos")]
#[repr(C)]
struct MessageBuffer {
    pub msg_magic: c_int,
    pub msg_size: c_int,
    pub msg_bufx: c_int,
    // write pointer
    pub msg_bufr: c_int,
    // read pointer
    pub msg_bufc: *mut u8,     // buffer
}

#[cfg(target_os = "macos")]
impl Default for MessageBuffer {
    fn default() -> MessageBuffer {
        MessageBuffer {
            msg_magic: 0,
            msg_size: 0,
            msg_bufx: 0,
            msg_bufr: 0,
            msg_bufc: ptr::null_mut() as *mut u8,
        }
    }
}

#[cfg(target_os = "macos")]
impl fmt::Debug for MessageBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MessageBuffer {{ magic: 0x{:x}, size: {}, bufx: {}}}", self.msg_magic, self.msg_size, self.msg_bufx)
    }
}

// this extern block links to the libproc library
// Original signatures of functions can be found at http://opensource.apple.com/source/Libc/Libc-594.9.4/darwin/libproc.c
#[cfg(target_os = "macos")]
#[link(name = "proc", kind = "dylib")]
extern {
    fn proc_kmsgbuf(buffer: *mut MessageBuffer, buffersize: u32) -> c_int;
}

/// faclev,seqnum,timestamp[optional, ...];message\n
///  TAGNAME=value
///  TAGNAME=value
/// Get a message from the kernel message buffer - as used by dmesg
// See http://opensource.apple.com//source/system_cmds/system_cmds-336.6/dmesg.tproj/dmesg.c
#[cfg(target_os = "macos")]
pub fn kmsgbuf() -> Result<String, String> {
    let mut message_buffer: MessageBuffer = Default::default();
    let ret: i32;

    unsafe {
        ret = proc_kmsgbuf(&mut message_buffer, mem::size_of::<MessageBuffer>() as u32);
    }

    if ret <= 0 {
        Err(helpers::get_errno_with_message(ret))
    } else if message_buffer.msg_magic != MSG_MAGIC {
        println!("Message buffer: {:?}", message_buffer);
        Err(format!("The magic number 0x{:x} is incorrect", message_buffer.msg_magic))
    } else {
        // Avoid starting beyond the end of the buffer
        if message_buffer.msg_bufx >= MAX_MSG_BSIZE {
            message_buffer.msg_bufx = 0;
        }
        let mut output: Vec<u8> = Vec::new();

        // The message buffer is circular; start at the read pointer, and go to the write pointer - 1.
        unsafe {
            let mut ch: u8;
            let mut p: *mut u8 = message_buffer.msg_bufc.offset(message_buffer.msg_bufx as isize);
            let ep: *mut u8 = message_buffer.msg_bufc.offset((message_buffer.msg_bufx - 1) as isize);

            while p != ep {
                // If at the end, then loop around to the start
                // TODO should use actual size (from struct element) - not the max size??
                if p == message_buffer.msg_bufc.offset(MAX_MSG_BSIZE as isize) {
                    p = message_buffer.msg_bufc;
                }

                ch = *p;
                output.push(ch);
                p = p.offset(1);
            }

            Ok(String::from_utf8(output).map_err(|_| "Could not convert to UTF-8")?)
        }
    }
}

#[cfg(target_os = "linux")]
pub fn kmsgbuf() -> Result<String, String> {
    let file = File::open("/dev/kmsg").map_err(|_| "Could not open /dev/kmsg file '{}'")?;
    let kmsg_channel = spawn_kmsg_channel(file);
    let duration = time::Duration::from_millis(1);
    let mut buf = String::new();
    loop {
        match kmsg_channel.recv_timeout(duration) {
            Ok(line) => buf.push_str(&line),
            _ => break,
        }
    }

    Ok(buf)
}

#[cfg(target_os = "linux")]
fn spawn_kmsg_channel(file: File) -> Receiver<String> {
    let mut reader = BufReader::new(file);
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut line = String::new();
        let _len = reader.read_line(&mut line).unwrap();
        println!("{}", line);
        tx.send(line).unwrap();
    });
    rx
}

#[cfg(test)]
mod test {
    use std::io;
    use std::io::Write;

    use crate::libproc::proc_pid::am_root;

    use super::kmsgbuf;

    #[test]
    #[ignore]
    // TODO fix on macos: an error message is returned - https://github.com/andrewdavidmackenzie/libproc-rs/issues/39
    // Message buffer: MessageBuffer { magic: 0x3a657461, size: 1986947360, bufx: 1684630625}
    // thread 'libproc::kmesg_buffer::test::kmessagebuffer_test' panicked at 'The magic number 0x3a657461 is incorrect', src/libproc/kmesg_buffer.rs:194:33
    fn kmessagebuffer_test() {
        if am_root() {
            match kmsgbuf() {
                Ok(buffer) => println!("Buffer: {:?}", buffer),
                Err(message) => panic!(message)
            }
        } else {
            writeln!(&mut io::stdout(), "test libproc::kmesg_buffer::kmessagebuffer_test ... skipped as it needs to be run as root").unwrap();
        }
    }
}