use crate::lexer::TokenType;

pub trait Eval {
    fn eval(&self) -> f64;
}

#[derive(Debug)]
pub struct Expr {
    operation: TokenType,
    term: Term,
    expr_opts: Vec<ExprOpt>,
}

impl Expr {
    pub fn new(operation: TokenType, term: Term, expr_opts: Vec<ExprOpt>) -> Self {
        Self {
            operation,
            term,
            expr_opts,
        }
    }
}

impl Eval for Expr {
    fn eval(&self) -> f64 {
        let mut temp = 0.0;
        let val = self.term.eval();

        match self.operation {
            TokenType::Add => temp += val,
            TokenType::Sub => temp -= val,
            _ => panic!("failed during evaluation. struct: Expr"),
        }

        temp += self.expr_opts.eval();
        temp
    }
}

#[derive(Debug)]
pub struct ExprOpt {
    operation: TokenType,
    term: Term,
}

impl ExprOpt {
    pub fn new(operation: TokenType, term: Term) -> Self {
        Self { operation, term }
    }
}

impl Eval for ExprOpt {
    fn eval(&self) -> f64 {
        let val = self.term.eval();

        match self.operation {
            TokenType::Add => return val,
            TokenType::Sub => return -val,
            _ => panic!("Unexpected tokentype for ExprOpt. Panicked during eval() of ExprOpt"),
        }
    }
}

impl Eval for Vec<ExprOpt> {
    fn eval(&self) -> f64 {
        let mut temp = 0.0;
        self.into_iter().for_each(|e| temp += e.eval());
        temp
    }
}

#[derive(Debug)]
pub struct Term {
    // operation: TokenType,
    pow: Pow,
    term_opts: Vec<TermOpt>,
}

impl Term {
    pub fn new(pow: Pow, term_opts: Vec<TermOpt>) -> Self {
        Self { pow, term_opts }
    }
}

impl Eval for Term {
    fn eval(&self) -> f64 {
        let mut temp = self.pow.eval();

        //not using iterator because there was some anoying borrowchecker errors
        for i in 0..self.term_opts.len() {
            match self.term_opts[i].operation {
                TokenType::Mult => temp *= self.term_opts[i].pow.eval(),
                TokenType::Div => temp /= self.term_opts[i].pow.eval(),
                _ => panic!("Unexpected token in TermOpt. panicked in eval() for Vec<TermOpt>"),
            }
        }

        temp
    }
}

#[derive(Debug)]
pub struct TermOpt {
    operation: TokenType,
    pow: Pow,
}

impl TermOpt {
    pub fn new(operation: TokenType, pow: Pow) -> Self {
        Self { operation, pow }
    }
}

#[derive(Debug)]
pub struct Pow {
    num: f64,
    pow_opts: Vec<PowOpts>,
}

impl Pow {
    pub fn new(num: f64, pow_opts: Vec<PowOpts>) -> Self {
        Self { num, pow_opts }
    }
}

impl Eval for Pow {
    fn eval(&self) -> f64 {
        self.num.powf(self.pow_opts.eval())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PowOpts {
    pub num: f64,
}

impl Eval for Vec<PowOpts> {
    fn eval(&self) -> f64 {
        if self.len() > 1 {
            let mut temp = self[0].num;

            for i in 1..self.len() {
                temp = temp.powf(self[i].num)
            }

            return temp;
        } else if self.len() == 1 {
            return self[0].num;
        } else {
            return 1.0; // if there is nothing in the vec, return 1, not 0, because everything to the power of 0 is 1.
        }
    }
}
