use std::env;

use std::io;
use std::process;

enum Token {}
impl Token {
    pub const DIGIT: &str = r"\d";
    pub const ALPHANUMERIC: &str = r"\w";
}

struct Pattern;
impl Pattern {
    fn is_positive_character_group(pattern: &str) -> bool {
        if pattern.len() < 3 {
            return false;
        }

        let has_valid_prefix = pattern.get(..2).unwrap().contains(|prefix_char| {
            return prefix_char == ']' || char::is_alphabetic(prefix_char);
        });
        let has_valid_suffix = pattern.ends_with("]");

        return has_valid_prefix && has_valid_suffix;
    }

    fn is_negative_character_group(pattern: &str) -> bool {
        return pattern.starts_with("[^") && pattern.ends_with("]");
    }
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match pattern {
        Token::DIGIT => {
            return input_line.contains(|input_char| {
                return char::is_digit(input_char, 10);
            })
        }
        Token::ALPHANUMERIC => {
            return input_line.contains(|input_char| {
                return char::is_alphabetic(input_char);
            })
        }
        pattern if Pattern::is_positive_character_group(pattern) => {
            let chars_to_match = pattern
                .strip_prefix("[")
                .expect("failed to strip the [ from the pattern")
                .strip_suffix("]")
                .expect("failed to strip the ] from the pattern")
                .chars();

            let result = chars_to_match.fold(false, |match_found, char_to_match| {
                if input_line.contains(char_to_match) {
                    return true;
                }

                return match_found;
            });

            return result;
        }
        pattern if Pattern::is_negative_character_group(pattern) => {
            let excluded_chars = pattern
                .strip_prefix("[^")
                .expect("failed to strip the [^ from the pattern")
                .strip_suffix("]")
                .expect("failed to strip the ] from the pattern")
                .chars();

            let has_excluded_chars =
                excluded_chars.fold(false, |excluded_char_found, excluded_char| {
                    if input_line.contains(excluded_char) {
                        return true;
                    }

                    return excluded_char_found;
                });

            return !has_excluded_chars;
        }
        _ => return input_line.contains(pattern),
    }
}

// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}

#[cfg(test)]
mod test {
    use std::{
        error::Error,
        io::Write,
        path::PathBuf,
        process::{Child, Command, Stdio},
    };

    #[test]
    fn validates_the_first_parameter() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(String::from("target/debug/grep-starter-rust"));
        let cmd = Command::new(path).args(["a"]).status()?;

        assert_eq!(cmd.code().unwrap(), 1);
        return Ok(());
    }

    #[test]
    fn single_character() -> Result<(), Box<dyn Error>> {
        {
            let mut cmd = spawn_cmd(vec!["-E", "a"])?;

            write!(cmd.stdin.as_mut().unwrap(), "{}", "apple")?;
            let output = cmd.wait_with_output()?;
            assert_eq!(output.status.code().unwrap(), 0);
        }

        {
            let mut cmd = spawn_cmd(vec!["-E", "d"])?;

            write!(cmd.stdin.as_mut().unwrap(), "{}", "apple")?;
            let output = cmd.wait_with_output()?;
            assert_eq!(output.status.code().unwrap(), 1);
        }

        return Ok(());
    }

    #[test]
    fn match_digits() -> Result<(), Box<dyn Error>> {
        {
            let mut cmd = spawn_cmd(vec!["-E", r"\d"])?;

            write!(cmd.stdin.as_mut().unwrap(), "{}", "apple123")?;
            let output = cmd.wait_with_output()?;
            assert_eq!(output.status.code().unwrap(), 0);
        }

        {
            let mut cmd = spawn_cmd(vec!["-E", r"\d"])?;

            write!(cmd.stdin.as_mut().unwrap(), "{}", "apple")?;
            let output = cmd.wait_with_output()?;
            assert_eq!(output.status.code().unwrap(), 1);
        }

        return Ok(());
    }

    #[test]
    fn match_alphanumeric() -> Result<(), Box<dyn Error>> {
        {
            let mut cmd = spawn_cmd(vec!["-E", r"\w"])?;

            write!(cmd.stdin.as_mut().unwrap(), "{}", "alpha-num3ric")?;
            let output = cmd.wait_with_output()?;
            assert_eq!(output.status.code().unwrap(), 0);
        }

        {
            let mut cmd = spawn_cmd(vec!["-E", r"\w"])?;

            write!(cmd.stdin.as_mut().unwrap(), "{}", "$!?")?;
            let output = cmd.wait_with_output()?;
            assert_eq!(output.status.code().unwrap(), 1);
        }

        return Ok(());
    }

    #[test]
    fn match_positive_character_groups() -> Result<(), Box<dyn Error>> {
        {
            let mut cmd = spawn_cmd(vec!["-E", "[abc]"])?;

            write!(cmd.stdin.as_mut().unwrap(), "{}", "apple")?;
            let output = cmd.wait_with_output()?;
            assert_eq!(output.status.code().unwrap(), 0);
        }

        {
            let mut cmd = spawn_cmd(vec!["-E", "[a]"])?;

            write!(cmd.stdin.as_mut().unwrap(), "{}", "apple")?;
            let output = cmd.wait_with_output()?;
            assert_eq!(output.status.code().unwrap(), 0);
        }

        {
            let mut cmd = spawn_cmd(vec!["-E", "[d]"])?;

            write!(cmd.stdin.as_mut().unwrap(), "{}", "apple")?;
            let output = cmd.wait_with_output()?;
            assert_eq!(output.status.code().unwrap(), 1);
        }

        {
            let mut cmd = spawn_cmd(vec!["-E", "[]"])?;

            write!(cmd.stdin.as_mut().unwrap(), "{}", "apple")?;
            let output = cmd.wait_with_output()?;
            assert_eq!(output.status.code().unwrap(), 1);
        }

        return Ok(());
    }

    #[test]
    fn match_negative_character_groups() -> Result<(), Box<dyn Error>> {
        {
            let mut cmd = spawn_cmd(vec!["-E", "[^abc]"])?;

            write!(cmd.stdin.as_mut().unwrap(), "{}", "apple")?;
            let output = cmd.wait_with_output()?;
            assert_eq!(output.status.code().unwrap(), 1);
        }

        {
            let mut cmd = spawn_cmd(vec!["-E", "[^d]"])?;

            write!(cmd.stdin.as_mut().unwrap(), "{}", "apple")?;
            let output = cmd.wait_with_output()?;
            assert_eq!(output.status.code().unwrap(), 0);
        }

        {
            let mut cmd = spawn_cmd(vec!["-E", "[^]"])?;

            write!(cmd.stdin.as_mut().unwrap(), "{}", "apple")?;
            let output = cmd.wait_with_output()?;
            assert_eq!(output.status.code().unwrap(), 0);
        }

        return Ok(());
    }

    fn spawn_cmd(args: Vec<&str>) -> Result<Child, Box<dyn Error>> {
        let path = PathBuf::from(String::from("target/debug/grep-starter-rust"));
        let cmd = Command::new(path)
            .args(args)
            .stdin(Stdio::piped())
            .spawn()?;

        return Ok(cmd);
    }
}
