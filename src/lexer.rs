#[derive(Debug, Clone, Copy)]
pub struct Token {
    token_type: TokenType,
    value: Option<f64>,
    start_pos: usize,
}

impl Token {
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn start_pos(&self) -> usize {
        self.start_pos
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }
}

pub struct Lexer {
    input: String,
    current_pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.trim().to_string(),
            current_pos: 0,
        }
    }

    /// Goes through input and converts it to a vec of tokens
    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.current_pos < self.input.len() {
            let current_char = self.input.chars().nth(self.current_pos).unwrap(); // Can never panic because current_pos is always less than the input length due to while loop

            match current_char {
                char if char.is_whitespace() => self.current_pos += 1,
                char if char.is_digit(10) => tokens.push(self.lex_num()),
                '+' => tokens.push(self.make_token(TokenType::Add)),
                '-' => tokens.push(self.make_token(TokenType::Sub)),
                '*' => tokens.push(self.make_token(TokenType::Mult)),
                '/' => tokens.push(self.make_token(TokenType::Div)),
                '^' => tokens.push(self.make_token(TokenType::Pow)),
                _ => panic!("unknown character {} at {}", current_char, self.current_pos),
            }
        }

        tokens.push(self.make_token(TokenType::EOF));

        tokens
    }

    fn lex_num(&mut self) -> Token {
        let text: String = self.input[self.current_pos..]
            .chars()
            .take_while(|c| c.is_digit(10) || *c == '.')
            .collect();

        let token = self.make_num_token(&text);

        self.current_pos += text.len();

        token
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        let token = Token {
            token_type,
            value: None,
            start_pos: self.current_pos,
        };

        self.current_pos += 1;

        token
    }

    /// Creates num-token WITHOUT setting the current curser position to the next char
    fn make_num_token(&self, text: &str) -> Token {
        Token {
            token_type: TokenType::Num,
            value: Some(text.parse().expect("Fucked up during parsing of int")),
            start_pos: self.current_pos,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    EOF,
    Num,
    Add,
    Sub,
    Mult,
    Div,
    Pow,
}
