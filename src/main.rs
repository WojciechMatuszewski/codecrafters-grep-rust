use std::env;

use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    println!("pattern: {:?}", pattern);
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
        process::{Command, Stdio},
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
            let path = PathBuf::from(String::from("target/debug/grep-starter-rust"));
            let mut cmd = Command::new(path)
                .args(["-E", "a"])
                .stdin(Stdio::piped())
                .spawn()?;

            write!(cmd.stdin.as_mut().unwrap(), "{}", "apple").unwrap();
            let output = cmd.wait_with_output()?;
            assert_eq!(output.status.code().unwrap(), 0);
        }

        {
            let path = PathBuf::from(String::from("target/debug/grep-starter-rust"));
            let mut cmd = Command::new(path)
                .args(["-E", "d"])
                .stdin(Stdio::piped())
                .spawn()?;

            write!(cmd.stdin.as_mut().unwrap(), "{}", "apple").unwrap();
            let output = cmd.wait_with_output()?;
            assert_eq!(output.status.code().unwrap(), 1);
        }

        return Ok(());
    }
}
