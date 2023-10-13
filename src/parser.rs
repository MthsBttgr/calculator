use crate::eval::{Expr, ExprOpt, Parenthesese, Pow, PowOpts, Term, TermOpt};
use crate::lexer::{
    Token,
    TokenType::{self, *},
};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    fn current_token(&self) -> Token {
        *self.tokens.get(self.index).expect("didn't get token")
    }

    fn current_token_with_option(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn parse(&mut self) -> Expr {
        let res = self.expr();

        match self.current_token().token_type() {
            EOF => return res,
            _ => panic!("Parsing ended before reaching end of calculation"),
        }
    }

    fn expr(&mut self) -> Expr {
        let op = match *self.current_token().token_type() {
            TokenType::Num => TokenType::Add,
            TokenType::Sub => {
                self.eat(Sub);
                TokenType::Sub
            }
            TokenType::LeftParen => TokenType::Add,
            // TokenType::LeftParen => {
            //     let vec = Vec::new();

            //     loop {
            //         if self.current_token().== TokenType::RightParen {

            //         }
            //     }
            // }
            _ => panic!(
                "idk, the first character isnt allowed ig. Token: {:#?}",
                self.current_token()
            ),
        };

        Expr::new(op, self.term(), self.expr_opts())
    }

    fn expr_opts(&mut self) -> Vec<ExprOpt> {
        let mut seq = Vec::new();

        while *self.current_token().token_type() == TokenType::Add
            || *self.current_token().token_type() == TokenType::Sub
        {
            seq.push(ExprOpt::new(
                *self.eat(*self.current_token().token_type()).token_type(),
                self.term(),
            ));
        }

        seq
    }

    fn term(&mut self) -> Term {
        Term::new(self.pow(), self.term_opts())
    }

    fn term_opts(&mut self) -> Vec<TermOpt> {
        let mut seq = Vec::new();

        while *self.current_token().token_type() == TokenType::Mult
            || *self.current_token().token_type() == TokenType::Div
        {
            seq.push(TermOpt::new(
                *self.eat(*self.current_token().token_type()).token_type(),
                self.pow(),
            ));
        }

        seq
    }

    fn pow(&mut self) -> Pow {
        Pow::new(self.parenthesese(), self.pow_opts())
    }

    fn pow_opts(&mut self) -> Vec<PowOpts> {
        let mut seq = Vec::new();

        while *self
            .current_token_with_option()
            .expect(&format!(
                "failed at: {}\n list of tokens: {:#?}",
                self.index, self.tokens
            ))
            .token_type()
            == TokenType::Pow
        {
            self.eat(*self.current_token().token_type());
            // let num = self.eat(TokenType::Num);
            seq.push(PowOpts {
                parenthesese: self.parenthesese(),
            });
        }

        seq
    }

    fn parenthesese(&mut self) -> Parenthesese {
        let mut num = None;
        let mut expr = Box::new(None);

        match self.current_token().token_type() {
            TokenType::Num => num = self.eat(TokenType::Num).value(),
            TokenType::LeftParen => {
                self.eat(TokenType::LeftParen);
                expr = Box::new(Some(self.generate_expr_inside_parenthesese()))
            }
            _ => panic!("panicked at unexpected token: {:#?}", self.current_token()),
        }

        Parenthesese::new(expr, num)
    }

    fn generate_expr_inside_parenthesese(&mut self) -> Expr {
        let mut tokens = Vec::new();

        let mut amount_of_inner_parenthesese = 0;

        loop {
            match self.current_token().token_type() {
                TokenType::LeftParen => {
                    tokens.push(self.eat(*self.current_token().token_type()));
                    amount_of_inner_parenthesese += 1;
                }
                TokenType::RightParen => {
                    if amount_of_inner_parenthesese == 0 {
                        self.eat(TokenType::RightParen);
                        break;
                    } else {
                        tokens.push(self.eat(*self.current_token().token_type()));
                        amount_of_inner_parenthesese -= 1;
                    }
                }
                TokenType::EOF => panic!(
                    "{}",
                    format!("No matching right parenthese. index: {}", self.index)
                ),
                //TokenType::LeftParen => //self.generate_expr_inside_parenthesese(),
                _ => tokens.push(self.eat(*self.current_token().token_type())),
            }
        }

        tokens.push(Token::new(TokenType::EOF, None, tokens.len()));

        Parser::new(tokens).expr()
    }

    fn eat(&mut self, token_type: TokenType) -> Token {
        let res = self.current_token();
        if *res.token_type() != token_type {
            panic!(
                "expected {:#?}, got {:#?} at position {:#?}, eat() function",
                token_type,
                res.token_type(),
                res.start_pos()
            )
        }

        self.index += 1;
        res
    }
}
