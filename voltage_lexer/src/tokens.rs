#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Identifier { val: Vec<char> },
    Plus { val: char },
    Minus { val: char },
    Multiplication { val: char },
    Division { val: char },
    Assign { val: char },
    Bang { val: char },
    Semicolon { val: char },
    Colon { val: char },
    LParen { val: char },
    RParen { val: char },
    Comma { val: char },
    LBrace { val: char },
    RBrace { val: char },

    // Cmp operatores
    Lt { val: char },
    Gt { val: char },
    LtOrEq { val: [char; 2] },
    GtOrEq { val: [char; 2] },
    Eq { val: [char; 2] },
    NotEq { val: [char; 2] },

    // Types
    Int { val: Vec<char> },
    Float { val: Vec<char> },
    String { val: Vec<char> },
    Arrow { val: String },
    Char { val: char },
    // KEYWORDS
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    End,
    Module,
    Unkown,
    Public,
    Whitespace,
    EOF,
}

impl Token {
    pub fn get_keyword_token(ident: &Vec<char>) -> Result<Token, String> {
        let identifier: String = ident.into_iter().collect();
        match &identifier[..] {
            "func" => Ok(Token::Function),
            "let" => Ok(Token::Let),
            "true" => Ok(Token::True),
            "false" => Ok(Token::False),
            "if" => Ok(Token::If),
            "else" => Ok(Token::Else),
            "end" => Ok(Token::End),
            "return" => Ok(Token::Return),
            "module" => Ok(Token::Module),
            "public" => Ok(Token::Public),
            _ => Err(String::from("Not a keyword")),
        }
    }
}
