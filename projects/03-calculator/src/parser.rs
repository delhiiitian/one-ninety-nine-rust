// Step 2: read the tokens and compute the answer, respecting
// precedence: brackets, then ^, then * / %, then + -.
//
// Each rule of the grammar is one function, and the functions call each
// other (and themselves) recursively. This is a "recursive descent" parser:
//
//   expression = term   (("+" | "-") term)*
//   term       = power  (("*" | "/" | "%") power)*
//   power      = unary  ("^" power)?            right-associative: 2^3^2 = 2^9
//   unary      = "-" unary | primary
//   primary    = number | "ans" | name "(" expression ")" | "(" expression ")"

use crate::lexer::Token;

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    ans: f64, // the previous answer, available as `ans`
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], ans: f64) -> Parser<'a> {
        Parser { tokens, pos: 0, ans }
    }

    /// Parses the whole input; leftover tokens are an error.
    pub fn parse(&mut self) -> Result<f64, String> {
        let value = self.expression()?;
        match self.peek() {
            None => Ok(value),
            Some(t) => Err(format!("unexpected {t:?} after the expression")),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }

    fn expression(&mut self) -> Result<f64, String> {
        let mut value = self.term()?;
        while let Some(op) = self.peek() {
            match op {
                Token::Plus => {
                    self.pos += 1;
                    value += self.term()?;
                }
                Token::Minus => {
                    self.pos += 1;
                    value -= self.term()?;
                }
                _ => break,
            }
        }
        Ok(value)
    }

    fn term(&mut self) -> Result<f64, String> {
        let mut value = self.power()?;
        loop {
            let op = match self.peek() {
                Some(Token::Star) => Token::Star,
                Some(Token::Slash) => Token::Slash,
                Some(Token::Percent) => Token::Percent,
                _ => break,
            };
            self.pos += 1;
            let rhs = self.power()?;
            value = match op {
                Token::Star => value * rhs,
                _ if rhs == 0.0 => return Err("division by zero".to_string()),
                Token::Slash => value / rhs,
                _ => value % rhs,
            };
        }
        Ok(value)
    }

    fn power(&mut self) -> Result<f64, String> {
        let base = self.unary()?;
        if self.peek() == Some(&Token::Caret) {
            self.pos += 1;
            let exponent = self.power()?; // recursion makes ^ right-associative
            return Ok(base.powf(exponent));
        }
        Ok(base)
    }

    fn unary(&mut self) -> Result<f64, String> {
        if self.peek() == Some(&Token::Minus) {
            self.pos += 1;
            return Ok(-self.unary()?);
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<f64, String> {
        match self.next().cloned() {
            Some(Token::Number(n)) => Ok(n),
            Some(Token::LParen) => {
                let value = self.expression()?;
                self.expect_rparen()?;
                Ok(value)
            }
            Some(Token::Name(name)) if name == "ans" => Ok(self.ans),
            Some(Token::Name(name)) if name == "pi" => Ok(std::f64::consts::PI),
            Some(Token::Name(name)) => {
                let function = lookup_function(&name)?;
                if self.next() != Some(&Token::LParen) {
                    return Err(format!("expected '(' after {name}"));
                }
                let arg = self.expression()?;
                self.expect_rparen()?;
                function(arg)
            }
            Some(t) => Err(format!("unexpected {t:?}")),
            None => Err("expression ended too early".to_string()),
        }
    }

    fn expect_rparen(&mut self) -> Result<(), String> {
        match self.next() {
            Some(Token::RParen) => Ok(()),
            _ => Err("missing ')'".to_string()),
        }
    }
}

/// Functions are stored as plain `fn` values: a name maps to code.
fn lookup_function(name: &str) -> Result<fn(f64) -> Result<f64, String>, String> {
    let f: fn(f64) -> Result<f64, String> = match name {
        "sqrt" => |x| {
            if x < 0.0 {
                Err("square root of a negative number".to_string())
            } else {
                Ok(x.sqrt())
            }
        },
        "abs" => |x| Ok(x.abs()),
        "round" => |x| Ok(x.round()),
        "sin" => |x| Ok(x.to_radians().sin()),
        "cos" => |x| Ok(x.to_radians().cos()),
        _ => return Err(format!("unknown function '{name}'")),
    };
    Ok(f)
}
