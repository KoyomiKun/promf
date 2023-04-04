#[derive(Debug, PartialEq, Eq)]
enum Token {
    Ident(String),
    Digit(u64),
    Assign,
    Lbrace,
    Rbrace,
    Dquot,
    Seperator,
    Floatp,

    Illegal,
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        match value.as_str() {
            "=" => Token::Assign,
            "{" => Token::Lbrace,
            "}" => Token::Rbrace,
            "\"" => Token::Dquot,
            "," => Token::Seperator,
            "." => Token::Floatp,
            _ => {
                if let Some(d) = is_digit(&value) {
                    Token::Digit(d)
                } else if let Some(_) = is_letter(&value) {
                    Token::Ident(value)
                } else {
                    Token::Illegal
                }
            }
        }
    }
}

impl Into<String> for Token {
    fn into(self) -> String {
        match self {
            Token::Assign => "=".to_string(),
            Token::Lbrace => "{".to_string(),
            Token::Rbrace => "}".to_string(),
            Token::Dquot => "\"".to_string(),
            Token::Seperator => ",".to_string(),
            Token::Floatp => ".".to_string(),
            Token::Ident(s) => s,
            Token::Digit(d) => d.to_string(),
            Token::Illegal => "ILLEGAL".to_string(),
        }
    }
}

// metrics_name{labels="A", lable2="B"} = 4
const BLANKS: [char; 4] = [' ', '\t', '\n', '\r'];

struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input }
    }
    pub fn iter(&self) -> LexerIter {
        LexerIter::new(self.input.as_str())
    }
}

struct LexerIter<'a> {
    input_size: usize,
    input_bytes: &'a [u8],
    pos: usize, // current token cursor
}

impl<'a> LexerIter<'a> {
    pub fn new(lex: &'a str) -> Self {
        Self {
            input_size: lex.len(),
            input_bytes: lex.as_bytes(),
            pos: 0,
        }
    }

    fn next_non_blank(&self) -> Option<usize> {
        for i in self.pos..self.input_size {
            if !Self::is_blank(self.input_bytes[i] as char) {
                return Some(i);
            }
        }
        None
    }

    // return the string and next start pos
    fn read_string(&self) -> Option<(String, usize)> {
        let mut s = String::new();

        for i in self.pos..self.input_size {
            let b = self.input_bytes[i] as char;
            if Self::is_blank(b) || Self::is_letter(b) {
                return Some((s, i));
            }
            s.push(b);
        }
        None
    }

    #[inline]
    fn is_blank(c: char) -> bool {
        BLANKS.contains(&c)
    }

    #[inline]
    fn is_letter(c: char) -> bool {
        ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_'
    }

    #[inline]
    fn is_digit(c: char) -> bool {
        ('0'..='9').contains(&c)
    }
}

impl Iterator for LexerIter<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // find the first elem which is not blank
        if let Some(pos) = self.next_non_blank() {
            self.pos = pos;
            if let Some((res, next_pos)) = self.read_string() {
                self.pos = next_pos;
                return Some(res.into());
            }
        }
        None
    }
}

fn is_digit(s: &str) -> Option<u64> {
    let mut res = 0;
    for c in s.as_bytes() {
        let u = c - '0' as u8;
        if !(0..=9).contains(&u) {
            return None;
        }

        res *= 10;
        res += u as u64
    }
    Some(res)
}

fn is_letter(s: &str) -> Option<&str> {
    for c in s.as_bytes() {
        let c = &(c.to_owned() as char);
        if !('a'..='z').contains(c) && !('A'..='Z').contains(c) && c != &'_' {
            println!("char {}", c);
            return None;
        }
    }

    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_test() {
        let code = r#"http_code_name{label_1 = "v1", label_2 = "v2" } = 1.12"#;

        let expected = vec![
            Token::Ident("http_code_name".to_string()),
            Token::Lbrace,
            Token::Ident("label_1".to_string()),
            Token::Assign,
            Token::Dquot,
            Token::Ident("v1".to_string()),
            Token::Dquot,
            Token::Seperator,
            Token::Ident("label_2".to_string()),
            Token::Assign,
            Token::Dquot,
            Token::Ident("v2".to_string()),
            Token::Dquot,
            Token::Rbrace,
            Token::Assign,
            Token::Digit(1),
            Token::Floatp,
            Token::Digit(12),
        ];

        let lexer = Lexer::new(code.to_string());
        let mut iter = lexer.iter().enumerate();

        while let Some((i, t)) = iter.next() {
            assert_eq!(expected[i], t)
        }
    }
}
