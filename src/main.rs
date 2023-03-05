use std::env;

use std::io;
use std::process;

fn recurse(input: &[char], pattern: &[char]) -> bool {
    if pattern.len() == 0 && input.len() == 0 {
        return true;
    }

    if pattern.len() != 0 && input.len() == 0 {
        return false;
    }

    if pattern.len() == 0 && input.len() != 0 {
        return true;
    }

    if pattern.get(0).unwrap() == input.get(0).unwrap() {
        let new_input = &input[1..];
        let new_pattern = &pattern[1..];
        return recurse(new_input, new_pattern);
    }

    match pattern {
        [first, second, ..] if first == &'\\' && second == &'d' => {
            let digit_position = input.iter().position(|x| return char::is_numeric(*x));
            match digit_position {
                Some(position) => {
                    let new_input = &input[position + 1..];
                    let new_pattern = &pattern[2..];
                    return recurse(new_input, new_pattern);
                }
                None => false,
            }
        }
        [first, second, ..] if first == &'\\' && second == &'w' => {
            let alphanumeric_position = input.iter().position(|x| return char::is_alphanumeric(*x));
            match alphanumeric_position {
                Some(position) => {
                    let new_input = &input[position + 1..];
                    let new_pattern = &pattern[2..];
                    return recurse(new_input, new_pattern);
                }
                None => false,
            }
        }
        [first, second, rest @ ..] if first == &'[' && second == &'^' => {
            if rest.starts_with(&[']']) {
                return true;
            }

            let pattern_end = rest.iter().position(|x| x == &']').unwrap();
            let negative_character_group = &rest[0..=pattern_end - 1];
            let character_to_match = input.get(0).unwrap();

            if negative_character_group.contains(character_to_match) {
                return false;
            }

            let new_input = &input[1..];
            let new_pattern = &pattern[pattern_end + 3..];
            return recurse(new_input, new_pattern);
        }
        [first, rest @ ..] if first == &'[' => {
            if rest.starts_with(&[']']) {
                return false;
            }

            let pattern_end = rest.iter().position(|x| x == &']').unwrap();
            let positive_character_group = &rest[0..=pattern_end - 1];
            let character_to_match = input.get(0).unwrap();

            if !positive_character_group.contains(character_to_match) {
                return false;
            }

            let new_input = &input[1..];
            let new_pattern = &pattern[pattern_end + 2..];
            return recurse(new_input, new_pattern);
        }
        _ => return false,
    }
}

fn match_pattern(input: &str, pattern: &str) -> bool {
    let filtered_input = input.chars().collect::<Vec<char>>();
    let filtered_pattern = pattern.chars().collect::<Vec<char>>();

    return recurse(filtered_input.as_slice(), filtered_pattern.as_slice());
}

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

    use crate::match_pattern;

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

    #[test]
    fn combine_character_classes() -> Result<(), Box<dyn Error>> {
        assert_eq!(match_pattern("1 apple", r"\d apple"), true);
        assert_eq!(match_pattern("1 apple", r"\d orange"), false);

        assert_eq!(match_pattern("100 apples", r"\d\d\d apples"), true);
        assert_eq!(match_pattern("1 apple", r"\d\d\d apple"), false);

        assert_eq!(match_pattern("3 dogs", r"\d \w\w\ws"), true);
        assert_eq!(match_pattern("4 cats", r"\d \w\w\ws"), true);
        assert_eq!(match_pattern("1 dog", r"\d \w\w\ws"), false);

        assert_eq!(match_pattern("1 dogx", r"\d \w\w\w[yxz]"), true);
        assert_eq!(match_pattern("1 dogx", r"\d \w\w\w[yz]"), false);

        assert_eq!(match_pattern("1 dogz", r"\d \w\w\w[yxz]"), true);
        assert_eq!(match_pattern("12 dogs", r"\d\d d[lol][^k]\w"), true);
        assert_eq!(match_pattern("12 dogs", r"\d\d d[ku][^k]\w"), false);

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
