use crate::errno::errno;
#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
use std::fs::File;
#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
use std::io::{BufRead, BufReader};

/// Helper function to get errno and return a String with the passed in `return_code`, the error
/// number and a possible message
pub fn get_errno_with_message(return_code: i32) -> String {
    let e = errno();
    let code = e.0;
    format!("return code = {return_code}, errno = {code}, message = '{e}'")
}

/// Helper function that depending on the `ret` value:
/// - is negative or 0, then form an error message from the `errno` value
/// - is positive, take `ret` as the length of the success message in `buf` in bytes
pub fn check_errno(ret: i32, buf: &mut Vec<u8>) -> Result<String, String> {
    if ret <= 0 {
        Err(get_errno_with_message(ret))
    } else {
        // `ret` mucg be greater than 0 here so no sign-loss
        #[allow(clippy::cast_sign_loss)]
        unsafe {
            buf.set_len(ret as usize);
        }

        match String::from_utf8(buf.clone()) {
            Ok(return_value) => Ok(return_value),
            Err(e) => Err(format!("Invalid UTF-8 sequence: {e}")),
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
/// A helper function for finding named fields in specific /proc FS files for processes
/// This will be more useful when implementing more linux functions
pub fn procfile_field(filename: &str, field_name: &str) -> Result<String, String> {
    const SEPARATOR: &str = ":";
    let line_header = format!("{field_name}{SEPARATOR}");

    // Open the file in read-only mode (ignoring errors).
    let file =
        File::open(filename).map_err(|_| format!("Could not open /proc file '{filename}'"))?;
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let line = line.map_err(|_| "Could not read file contents")?;
        if line.starts_with(&line_header) {
            let parts: Vec<&str> = line.split(SEPARATOR).collect();
            return Ok(parts[1].trim().to_owned());
        }
    }

    Err(format!(
        "Could not find the field named '{field_name}' in the /proc FS file name '{filename}'"
    ))
}

#[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
/// Parse a memory amount string into integer number of bytes
/// e.g. 220844 kB -->
pub fn parse_memory_string(line: &str) -> Result<u64, String> {
    let parts: Vec<&str> = line.trim().split(' ').collect();
    if parts.is_empty() {
        return Err(format!("Could not parse Memory String: {line}"));
    }
    let multiplier: u64 = if parts.len() == 2 {
        match parts[1] {
            "MB" => 1024 * 1024,
            "kB" => 1024,
            "B" => 1,
            _ => return Err(format!("Could not parse units of Memory String: {line}")),
        }
    } else {
        1
    };

    let value: u64 = parts[0]
        .parse()
        .map_err(|_| "Could not parse value as integer")?;

    Ok(value * multiplier)
}

#[cfg(test)]
mod test {
    use super::check_errno;
    use crate::errno::{set_errno, Errno};

    #[cfg(any(target_os = "linux", target_os = "redox", target_os = "android"))]
    mod linux {
        use crate::libproc::helpers::parse_memory_string;

        #[test]
        fn test_valid_memory_string() {
            assert_eq!(parse_memory_string("220844 kB"), Ok(226144256));
        }

        #[test]
        fn test_valid_memory_string_spaces() {
            assert_eq!(parse_memory_string("  220844 kB  "), Ok(226144256));
        }

        #[test]
        fn test_invalid_memory_string_units() {
            assert!(parse_memory_string("  220844 THz  ").is_err());
        }

        #[test]
        fn test_invalid_memory_string() {
            assert!(parse_memory_string("    ").is_err());
        }

        #[test]
        fn test_invalid_memory_string_empty() {
            assert!(parse_memory_string("gobble dee gook").is_err())
        }
    }

    #[test]
    fn invalid_utf8() {
        let mut buf: Vec<u8> = vec![255, 0, 0];

        // Test - small test buffer so no problem truncating
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        if let Err(msg) = check_errno(buf.len() as i32, &mut buf) {
            assert!(msg.contains(
                "Invalid UTF-8 sequence: invalid utf-8 sequence of 1 bytes from index 0"
            ));
        }
    }

    #[test]
    fn positive_ret() {
        let message = "custom message";
        let mut buf: Vec<u8> = Vec::from(message.as_bytes());

        // Test - small test buffer so no problem truncating
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        if let Ok(msg) = check_errno(buf.len() as i32, &mut buf) {
            assert!(msg.contains(message));
        }
    }

    #[test]
    fn negative_ret() {
        let mut buf: Vec<u8> = vec![];
        set_errno(Errno(-1));

        // Test
        if let Err(mes) = check_errno(-1, &mut buf) {
            assert!(mes.contains("return code = -1, errno = -1"));
        }
    }

    #[test]
    fn zero_ret() {
        let mut buf: Vec<u8> = vec![];
        set_errno(Errno(2));

        // Test
        if let Err(mes) = check_errno(0, &mut buf) {
            assert!(mes.contains("return code = 0, errno = 2"));
        }
    }
}
