// Step 1: turn text like "2 * (3 + 4)" into tokens:
//   [Number(2), Star, LParen, Number(3), Plus, Number(4), RParen]

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Name(String), // a function name like sqrt, or the word ans
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    LParen,
    RParen,
}

pub fn tokenize(text: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = text.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
        } else if c.is_ascii_digit() || c == '.' {
            // Read a whole number such as 3.14
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }
            let text: String = chars[start..i].iter().collect();
            let value = text.parse().map_err(|_| format!("bad number '{text}'"))?;
            tokens.push(Token::Number(value));
        } else if c.is_alphabetic() {
            let start = i;
            while i < chars.len() && chars[i].is_alphanumeric() {
                i += 1;
            }
            tokens.push(Token::Name(chars[start..i].iter().collect()));
        } else {
            let token = match c {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' | '×' => Token::Star,
                '/' | '÷' => Token::Slash,
                '%' => Token::Percent,
                '^' => Token::Caret,
                '(' => Token::LParen,
                ')' => Token::RParen,
                other => return Err(format!("unexpected character '{other}'")),
            };
            tokens.push(token);
            i += 1;
        }
    }
    Ok(tokens)
}
