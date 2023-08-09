use std::env::args;

fn main() {
    let arguments: Vec<_> = args().collect();

    match arguments.len() {
        0 | 1 => eprintln!("Please provide one argument to kebab case."),
        2 => {
            let arg = &arguments[1];
            let kebabed_arg = kebab(arg);
            println!("{}", kebabed_arg);
        }
        _ => eprintln!("Too many arguments. Please provide one argument to kebab case."),
    }
}

fn kebab(s: &str) -> String {
    let mut res = String::new();

    for c in s.bytes() {
        match c {
            b'.' => {
                if res.ends_with('-') {
                    res.pop();
                }
                res.push(c as char);
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => res.push((c as char).to_ascii_lowercase()),
            b'-' | b'_' | b' ' | b'\r' | b'\n' | b'\t' => {
                if !res.ends_with('-') {
                    res.push('-')
                }
            }
            _ => {}
        }
    }

    while res.ends_with('-') {
        res.pop();
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignore_dashes_before_dot() {
        assert_eq!(kebab("test---.txt"), "test.txt");
    }

    #[test]
    fn save_one_dash_if_no_dot() {
        assert_eq!(kebab("test---txt"), "test-txt");
    }

    #[test]
    fn ignore_unicode() {
        assert_eq!(kebab("test💩.txt"), "test.txt");
    }

    #[test]
    fn kebab_spaces() {
        assert_eq!(kebab("first second"), "first-second");
    }

    #[test]
    fn kebab_multi_spaces() {
        assert_eq!(kebab("first \t\r\n   second"), "first-second");
    }

    #[test]
    fn ends_with_space() {
        assert_eq!(kebab("first second\t\r\n"), "first-second");
    }
}
