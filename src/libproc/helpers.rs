extern crate errno;

use self::errno::errno;

/*
    Helper function to get errno
*/
pub fn get_errno_with_message(ret: i32) -> String {
    let e = errno();
    let code = e.0 as i32;
    format!("return code = {}, errno = {}, message = '{}'", ret, code, e)
}

/*
    Helper function that checks the error number and depending on the value converts a returned
    buffer to UTF String and returns that as the successful Result
*/
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