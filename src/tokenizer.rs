use std::error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    UnknownChar(char),
}

//This should keep whitespace!
pub fn tokenize(s: &str) -> Result<Vec<String>> {
    let mut idx = 0;
    let mut tokens = vec![];

    let chars = s.trim().as_bytes();

    while idx < chars.len() {
        match chars[idx] {
            c if "<>[]{}(),:+*/&|!%$@=^".contains(c as char) => {
                tokens.push((c as char).to_string());
                idx += 1;
            }
            c if (c as char).is_numeric() => {
                let mut token = vec![];
                while idx < chars.len() {
                    if (chars[idx] as char).is_numeric() {
                        token.push(chars[idx]);
                        idx += 1;
                    } else {
                        break;
                    }
                }
                //All of this parsing
                let number = String::from_utf8(token).expect("Failed to create utf8 string");
                tokens.push(number);
            }
            c if (c as char).is_alphabetic() => {
                let mut token = vec![];
                while idx < chars.len() {
                    if (chars[idx] as char).is_alphabetic() {
                        token.push(chars[idx]);
                        idx += 1;
                    } else {
                        break;
                    }
                }
                tokens.push(String::from_utf8(token).expect("Failed to create utf8 string"));
            }
            c if (c as char).is_whitespace() => {
                if c == ('\n' as u8) {
                    tokens.push((c as char).to_string());
                }
                idx += 1;
            }
            c => return Err(Error::UnknownChar(c as char)),
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn brackets() {
        let tokens = tokenize("(hello ) mate").unwrap();
        let ans: Vec<String> = vec!["(", "hello", ")", "mate"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(tokens, ans);
    }

    #[test]
    fn if_statement() {
        let tokens = tokenize("if x > y then").unwrap();
        let ans: Vec<String> = vec!["if", "x", ">", "y", "then"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(tokens, ans);
    }

    #[test]
    fn whitespace() {
        let s = indoc!(
            "
            if x > y then
                5
        "
        );
        let tokens = tokenize(s).unwrap();
        let ans: Vec<String> = vec!["if", "x", ">", "y", "then", "\n", "5"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(tokens, ans);
    }

}
