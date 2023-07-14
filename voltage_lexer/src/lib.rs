use tokens::Token;

pub mod tokens;

#[derive(Debug)]
pub struct Lexer {
    pub input: Vec<char>,
    pub position: usize,
    pub line: usize,
    pub read_position: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(source: Vec<char>) -> Self {
        Self {
            input: source,
            position: 0,
            line: 0,
            read_position: 0,
            ch: ' ',
        }
    }

    pub fn lex(&mut self) -> Vec<tokens::Token> {
        let mut tokens = vec![];
        self.read_char();
        loop {
            let token = self.next_token();
            if token == tokens::Token::EOF {
                break;
            } else {
                if token != tokens::Token::Whitespace && token != tokens::Token::Unkown {
                    tokens.push(token);
                }
            }
        }
        tokens
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    pub fn read_char_back(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position - 2];
        }
        self.position = self.read_position;
        self.read_position = self.read_position - 1;
    }

    pub fn skip_whitespace(&mut self) {
        let ch = self.ch;
        if ch.is_whitespace() {
            self.read_char();
        }
    }

    fn token_match(&mut self) -> tokens::Token {
        let read_identifier = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && l.ch.is_alphanumeric() {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let read_number = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;

            while l.position < l.input.len() {
                if l.ch.is_numeric() || l.ch == '.' {
                    l.read_char();
                } else {
                    break;
                }
            }

            l.input[position..l.position].to_vec()
        };

        let token: tokens::Token;
        self.skip_whitespace();

        match self.ch {
            '=' => {
                self.read_char();
                if self.ch == '=' {
                    token = tokens::Token::Eq { val: ['=', '='] }
                } else {
                    self.read_char_back();
                    token = tokens::Token::Assign { val: self.ch };
                }
            }
            '+' => {
                token = tokens::Token::Plus { val: self.ch };
            }
            '-' => {
                let cur_ch = self.ch;
                self.read_char();
                if self.ch == '>' {
                    token = tokens::Token::Arrow {
                        val: [cur_ch, self.ch].into_iter().collect(),
                    }
                } else {
                    token = tokens::Token::Minus { val: self.ch };
                }
            }
            '!' => {
                self.read_char();
                if self.ch == '=' {
                    token = tokens::Token::NotEq { val: ['!', '='] }
                } else {
                    self.read_char_back();
                    token = tokens::Token::Bang { val: self.ch };
                }                
            }
            '/' => {
                token = tokens::Token::Division { val: self.ch };
            }
            '*' => {
                token = tokens::Token::Multiplication { val: self.ch };
            }
            '<' => {
                self.read_char();
                if self.ch == '=' {
                    token = tokens::Token::LtOrEq { val: ['<', '='] }
                } else {
                    self.read_char_back();
                    token = tokens::Token::Lt { val: self.ch };
                }
            }
            '>' => {
                self.read_char();
                if self.ch == '=' {
                    token = tokens::Token::GtOrEq { val: ['>', '='] }
                } else {
                    self.read_char_back();
                    token = tokens::Token::Gt { val: self.ch };
                }
            }
            ';' => {
                token = tokens::Token::Semicolon { val: self.ch };
            }
            ':' => {
                token = tokens::Token::Colon { val: self.ch };
            }
            '(' => {
                token = tokens::Token::LParen { val: self.ch };
            }
            ')' => {
                token = tokens::Token::RParen { val: self.ch };
            }
            ',' => {
                token = tokens::Token::Comma { val: self.ch };
            }
            '{' => {
                token = tokens::Token::LBrace { val: self.ch };
            }
            '}' => {
                token = tokens::Token::RBrace { val: self.ch };
            }
            '\0' => {
                token = tokens::Token::EOF;
            }
            _ if self.ch == '"' => {
                let mut stri: Vec<char> = vec![];
                self.read_char();
                while self.ch != '"' {
                    if self.ch == '\\' {
                        self.read_char();
                        stri.push(self.ch);
                    }
                    stri.push(self.ch);
                    self.read_char();
                }
                self.read_char();
                return tokens::Token::String { val: stri };
            }
            _ if self.ch == '\'' => {
                self.read_char();
                let val = self.ch.clone();
                self.read_char();

                self.read_char();

                return tokens::Token::Char { val };
            }
            _ if self.ch.is_numeric() => {
                let ident: Vec<char> = read_number(self);
                if ident.contains(&'.') {
                    return tokens::Token::Float { val: ident };
                }

                return tokens::Token::Int { val: ident };
            }
            _ if self.ch.is_ascii_alphanumeric() => {
                let ident: Vec<char> = read_identifier(self);
                match Token::get_keyword_token(&ident) {
                    Ok(keywork_token) => {
                        return keywork_token;
                    }
                    Err(_err) => {
                        return tokens::Token::Identifier { val: ident };
                    }
                }
            }
            _ => {
                if self.ch == '\n' || self.ch == '\n' || self.ch == '\r' {
                    self.line += 1;
                    self.position = 0;
                    // self.read_position = 0;
                }

                if self.ch == '\n' || self.ch == '\n' || self.ch == '\r' || self.ch == ' ' {
                    return tokens::Token::Whitespace;
                }
                panic!(
                    "[LEXER] Error: Unknown token found @ position {} '{}'",
                    self.position, self.ch
                );
                // return tokens::Token::Unkown;
            }
        }

        self.read_char();
        token
    }

    pub fn next_token(&mut self) -> tokens::Token {
        let mut t = self.token_match();
        if matches!(t, tokens::Token::Whitespace) {
            t = self.token_match()
        }
        t
    }
}
