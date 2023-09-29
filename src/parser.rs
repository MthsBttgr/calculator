use crate::eval::{Expr, ExprOpt, Pow, PowOpts, Term, TermOpt};
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

    fn lookahead(&self) -> Token {
        *self.tokens.get(self.index).unwrap()
    }

    pub fn parse(&mut self) -> Expr {
        let res = self.expr();

        match self.lookahead().token_type() {
            EOF => return res,
            _ => panic!("Parsing ended before reaching end of calculation"),
        }
    }

    fn expr(&mut self) -> Expr {
        let op = match *self.lookahead().token_type() {
            TokenType::Num => TokenType::Add,
            TokenType::Sub => {
                self.eat(Sub);
                TokenType::Sub
            }
            _ => panic!("idk, the first character isnt allowed ig"),
        };

        Expr::new(op, self.term(), self.expr_opts())
    }

    fn expr_opts(&mut self) -> Vec<ExprOpt> {
        let mut seq = Vec::new();

        while *self.lookahead().token_type() == TokenType::Add
            || *self.lookahead().token_type() == TokenType::Sub
        {
            seq.push(ExprOpt::new(
                *self.eat(*self.lookahead().token_type()).token_type(),
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

        while *self.lookahead().token_type() == TokenType::Mult
            || *self.lookahead().token_type() == TokenType::Div
        {
            seq.push(TermOpt::new(
                *self.eat(*self.lookahead().token_type()).token_type(),
                self.pow(),
            ));
        }

        seq
    }

    fn pow(&mut self) -> Pow {
        Pow::new(self.eat(TokenType::Num).value().unwrap(), self.pow_opts())
    }

    fn pow_opts(&mut self) -> Vec<PowOpts> {
        let mut seq = Vec::new();

        while *self.lookahead().token_type() == TokenType::Pow {
            self.eat(*self.lookahead().token_type());
            let num = self.eat(TokenType::Num);
            seq.push(PowOpts {
                num: num.value().unwrap(),
            });
        }

        seq
    }

    fn eat(&mut self, token_type: TokenType) -> Token {
        let mut res = self.lookahead();

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
