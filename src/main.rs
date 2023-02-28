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
    use assert_cmd::Command;
    use std::error::Error;

    #[test]
    fn validates_the_first_parameter() -> Result<(), Box<dyn Error>> {
        let mut cmd = Command::cargo_bin("grep-starter-rust")?;
        cmd.arg("a").write_stdin("apple").assert().failure().code(1);

        return Ok(());
    }

    #[test]
    fn single_character() -> Result<(), Box<dyn Error>> {
        {
            let mut cmd = Command::cargo_bin("grep-starter-rust")?;
            cmd.arg("-E")
                .arg("a")
                .write_stdin("apple")
                .assert()
                .success()
                .code(0);
        }

        {
            let mut cmd = Command::cargo_bin("grep-starter-rust")?;
            cmd.arg("-E")
                .arg("d")
                .write_stdin("apple")
                .assert()
                .failure()
                .code(1);
        }

        return Ok(());
    }
}
