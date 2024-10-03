use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
struct UnpackError;

impl fmt::Display for UnpackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid input")
    }
}

impl Error for UnpackError {}

fn handle_repeated_char(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    current_char: char,
    result: &mut String,
) -> Result<(), UnpackError> {
    if let Some(&next_char) = chars.peek() {
        if next_char.is_digit(10) {
            let mut count_str = String::new();
            while let Some(&digit) = chars.peek() {
                if digit.is_digit(10) {
                    count_str.push(digit);
                    chars.next();
                } else {
                    break;
                }
            }
            let count: usize = count_str.parse().unwrap();
            if count == 0 {
                return Err(UnpackError);
            }
            result.push_str(&current_char.to_string().repeat(count));
        } else {
            result.push(current_char);
        }
    } else {
        result.push(current_char);
    }
    Ok(())
}

fn unpack_string(s: &str) -> Result<String, UnpackError> {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    let mut escape = false;

    while let Some(c) = chars.next() {
        if escape {
            escape = false;
            if c != '\\' && !c.is_digit(10) {
                return Err(UnpackError);
            }

            handle_repeated_char(&mut chars, c, &mut result)?;
            continue;
        }

        if c == '\\' {
            if chars.peek().is_none() {
                return Err(UnpackError);
            }
            escape = true;
            continue;
        }

        if c.is_digit(10) {
            return Err(UnpackError);
        }

        handle_repeated_char(&mut chars, c, &mut result)?;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpack_string() {
        assert_eq!(unpack_string("a4bc2d5e").unwrap(), "aaaabccddddde");
        assert_eq!(unpack_string("abcd").unwrap(), "abcd");
        assert!(unpack_string("45").is_err());
        assert_eq!(unpack_string("").unwrap(), "");
    }

    #[test]
    fn test_unpack_with_escape() {
        assert_eq!(unpack_string(r"qwe\4\5").unwrap(), "qwe45");
        assert_eq!(unpack_string(r"qwe\45").unwrap(), "qwe44444");
        assert_eq!(unpack_string(r"qwe\\5").unwrap(), r"qwe\\\\\");
    }

    #[test]
    fn test_invalid_string() {
        assert!(unpack_string("a4bc2d0e").is_err());
        assert!(unpack_string("123").is_err());
        assert!(unpack_string(r"a\").is_err());
    }
}

fn main() {
    match unpack_string("a4bc2d5e") {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Error: {}", e),
    }
}
