use crate::errno::errno;

/*
    Helper function to get errno
*/
pub fn get_errno_with_message(ret: i32) -> String {
    let e = errno();
    let code = e.0 as i32;
    format!("return code = {}, errno = {}, message = '{}'", ret, code, e)
}

/// Helper function that depending on the `ret` value:
/// - is negative or 0, then form an error message from the `errno` value
/// - is positive, take `ret` as the length of the success message in `buf` in bytes
pub fn check_errno(ret: i32, buf: &mut Vec<u8>) -> Result<String, String> {
    if ret <= 0 {
        Err(get_errno_with_message(ret))
    } else {
        unsafe {
            buf.set_len(ret as usize);
        }

        match String::from_utf8(buf.to_vec()) {
            Ok(return_value) => Ok(return_value),
            Err(e) => Err(format!("Invalid UTF-8 sequence: {}", e))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::errno::{set_errno, Errno};
    use super::check_errno;

    #[test]
    fn invalid_utf8() {
        let mut buf: Vec<u8> = vec!(255, 0, 0);

        // Test
        if let Err(msg) = check_errno(buf.len() as i32, &mut buf) {
            assert_eq!(msg, "Invalid UTF-8 sequence: invalid utf-8 sequence of 1 bytes from index 0")
        }
    }

    #[test]
    fn positive_ret() {
        let message = "custom message";
        let mut buf: Vec<u8> = Vec::from(message.as_bytes());

        // Test
        if let Ok(msg) = check_errno(buf.len() as i32, &mut buf) {
            assert_eq!(msg, message);
        }
    }

    #[test]
    fn negative_ret() {
        let mut buf: Vec<u8> = vec!();
        set_errno(Errno(-1));

        // Test
        if let Err(mes) = check_errno(-1, &mut buf) {
            #[cfg(target_os = "macos")]
            assert_eq!(mes, "return code = -1, errno = -1, message = 'Unknown error: -1'");
            #[cfg(target_os = "linux")]
            assert_eq!(mes, "return code = -1, errno = -1, message = 'Unknown error -1'");
        }
    }

    #[test]
    fn zero_ret() {
        let mut buf: Vec<u8> = vec!();
        set_errno(Errno(2));

        // Test
        if let Err(mes) = check_errno(0, &mut buf) {
            assert_eq!(mes, "return code = 0, errno = 2, message = 'No such file or directory'")
        }
    }
}