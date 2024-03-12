#[cfg(target_os = "macos")]
use std::str;

#[cfg(target_os = "macos")]
use libc::c_void;

#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
use std::fs::File;
#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
use std::io::{BufRead, BufReader};
#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
use std::sync::mpsc;
#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
use std::sync::mpsc::Receiver;
#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
use std::{thread, time};

#[cfg(target_os = "macos")]
use crate::osx_libproc_bindings::{proc_kmsgbuf, MAXBSIZE as MAX_MSG_BSIZE};

#[cfg(any(target_os = "macos", doc))]
/// Read messages from the kernel message buffer
///
/// Entries are in the format:
/// faclev,seqnum,timestamp[optional, ...];message\n
///  TAGNAME=value (0 or more Tags)
/// See <http://opensource.apple.com//source/system_cmds/system_cmds-336.6/dmesg.tproj/dmesg.c>
///
/// On linux:
/// Turns out that reading to the end of an "infinite file" like "/dev/kmsg" with standard file
/// reading methods will block at the end of file, so a workaround is required. Do the blocking
/// reads on a thread that sends lines read back through a channel, and then return when the thread
/// has blocked and can't send anymore. Returning will end the thread and the channel.
///
/// # Errors
///
/// An `Err` will be returned if `/dev/kmsg` device cannot be read
#[cfg(target_os = "macos")]
pub fn kmsgbuf() -> Result<String, String> {
    let mut message_buffer: Vec<u8> = Vec::with_capacity(MAX_MSG_BSIZE as _);
    let buffer_ptr = message_buffer.as_mut_ptr().cast::<c_void>();
    let ret: i32;

    unsafe {
        // This assumes that MAX_MSG_BSIZE < u32::MAX - but compile time asserts are experimental
        #[allow(clippy::cast_possible_truncation)]
        let buffersize = message_buffer.capacity() as u32;
        ret = proc_kmsgbuf(buffer_ptr, buffersize);
        if ret > 0 {
            // `ret` cannot be negative here - so cannot lose the sign
            #[allow(clippy::cast_sign_loss)]
            message_buffer.set_len(ret as usize - 1);
        }
    }

    if message_buffer.is_empty() {
        Err("Could not read kernel message buffer".to_string())
    } else {
        let msg = str::from_utf8(&message_buffer)
            .map_err(|_| "Could not convert kernel message buffer from utf8".to_string())?
            .parse()
            .map_err(|_| "Could not parse kernel message")?;
        Ok(msg)
    }
}

#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
pub fn kmsgbuf() -> Result<String, String> {
    let mut file = File::open("/dev/kmsg");
    if file.is_err() {
        file = File::open("/dev/console");
    }
    let file = file.map_err(|_| "Could not open /dev/kmsg nor /dev/console file '{}'")?;
    let kmsg_channel = spawn_kmsg_channel(file);
    let duration = time::Duration::from_millis(1);
    let mut buf = String::new();
    while let Ok(line) = kmsg_channel.recv_timeout(duration) {
        buf.push_str(&line);
    }

    Ok(buf)
}

// Create a channel to return lines read from a file on, then create a thread that reads the lines
// and sends them back on the channel one by one. Eventually it will get to EOF or block
#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
fn spawn_kmsg_channel(file: File) -> Receiver<String> {
    let mut reader = BufReader::new(file);
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(_) => {
                if tx.send(line).is_err() {
                    break;
                }
            }
            _ => break,
        }
    });

    rx
}

#[cfg(test)]
mod test {
    use crate::libproc::proc_pid::am_root;

    use super::kmsgbuf;

    #[test]
    fn kmessage_buffer_test() {
        if am_root() {
            match kmsgbuf() {
                Ok(_) => {}
                Err(message) => panic!("{}", message),
            }
        } else {
            println!("test skipped as it needs to be run as root");
        }
    }
}
