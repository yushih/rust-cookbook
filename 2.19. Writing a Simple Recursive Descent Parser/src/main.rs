use std::iter::Peekable;

#[macro_use]
extern crate lazy_static;

// The lexer
#[derive(Debug, PartialEq, Eq)]
enum Token {
    Add,
    Sub,
    Mul,
    Div,
    Num(i64),
    LParen,
    RParen,
    Ws
}

#[derive(Clone, Copy)]
struct Lexer<'a> {
    input: &'a str,
    cursor: usize
}

impl<'a> Lexer<'a> {
    pub fn new (input: &'a str) -> Self {
        Lexer {input: input, cursor: 0}
    }
}


impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    
    fn next (&mut self) -> Option<Self::Item> {
        static TOKENS : [(&str, &(Fn(&str)->Result<Token, &str> + Sync)); 8] = [
            (r#"\d+"#, &|t|t.parse::<i64>().map(|i|Token::Num(i)).map_err(|_|"parse i64")),
            (r#"\+"#,  &|_|Ok(Token::Add)),
            (r#"-"#,   &|_|Ok(Token::Sub)),
            (r#"\*"#,  &|_|Ok(Token::Mul)),
            (r#"/"#,   &|_|Ok(Token::Div)),
            (r#"\("#,  &|_|Ok(Token::LParen)),
            (r#"\)"#,  &|_|Ok(Token::RParen)),
            (r#"\s+"#, &|_|Ok(Token::Ws)),
        ];
        lazy_static! {
            static ref R : regex::Regex = regex::Regex::new(&TOKENS.iter().map(|( pat, _)|format!("(^{})", pat)).collect::<Vec<_>>().join("|")).unwrap();
        }

        if let Some(captures) = R.captures(&self.input[self.cursor..]) {
            for (i, (_, conv)) in TOKENS.iter().enumerate() {
                if let Some(m) = captures.get(i+1) {
                    self.cursor += m.end();
                    if let Ok(tkn) = conv(m.as_str()) {
                        return Some(tkn);
                    } else {
                        return None;
                    }
                }
            }
            panic!("should capture a token");
        } else {
            None
        }
    }
}

// The parser
trait ExprVisitor {
    type Expr;
    type Term;
    type Factor;
    
    /* expr ::= expr + term   */ fn add (&self, l: Self::Expr, r: Self::Term) -> Self::Expr;
    /*      |   expr - term   */ fn sub (&self, l: Self::Expr, r: Self::Term) -> Self::Expr;
    /*      |   term          */ fn term_expr (&self, t: Self::Term) -> Self::Expr;
    /* term ::= term * factor */ fn mul (&self, l: Self::Term, r: Self::Factor) -> Self::Term;
    /*      |   term / factor */ fn div (&self, l: Self::Term, r: Self::Factor) -> Self::Term;
    /*      |   factor        */ fn factor_term (&self, f: Self::Factor) -> Self::Term;
    /* factor ::= ( expr )    */ fn paren (&self, e: Self::Expr) -> Self::Factor;
    /*        |   NUM         */ fn num (&self, i: i64) -> Self::Factor;
}

fn expect<'a, T:PartialEq>(next: &T, domain: &[T], err: &'a str) -> Result<(), &'a str> {
    if let Some(_) = domain.iter().find(|x|*x==next) {
        Ok(())
    } else {
        Err(err)
    }
}

fn parse_expr<'a, V:ExprVisitor, L:Iterator<Item=Token>> (lexer: &mut Peekable<L>, visitor: &V) -> Result<<V as ExprVisitor>::Expr, &'a str> {
    // expression ::= term { ('+'|'-') term }*"
    let mut left = visitor.term_expr(parse_term(lexer, visitor)?);
    loop{
        if lexer.peek()==Some(&Token::Add) || lexer.peek()==Some(&Token::Sub) {
            let next = lexer.next().unwrap();
            let right = parse_term(lexer, visitor)?;
            if next == Token::Add {
                left = visitor.add(left, right);
            } else {
                left = visitor.sub(left, right);
            }
        } else {
            break;
        }
    } 
    Ok(left)
}

fn parse_term<'a, V:ExprVisitor, L:Iterator<Item=Token>> (lexer: &mut Peekable<L>, visitor: &V) -> Result<<V as ExprVisitor>::Term, &'a str> {
    // term ::= factor { ('*'|'/') factor }*
    let mut left = visitor.factor_term(parse_factor(lexer, visitor)?);
    
    loop {
        if lexer.peek()==Some(&Token::Mul) || lexer.peek()==Some(&Token::Div) {
            let next = lexer.next().unwrap();
            let right = parse_factor(lexer, visitor)?;
            if next == Token::Mul {
                left = visitor.mul(left, right);
            } else {
                left = visitor.div(left, right);
            }
        } else {
            break;
        }
    }
    Ok(left)
}

fn parse_factor<'a, V:ExprVisitor, L:Iterator<Item=Token>> (lexer: &mut Peekable<L>, visitor: &V) -> Result<<V as ExprVisitor>::Factor, &'a str> {
    // factor ::= NUM | ( expr )
    let next = lexer.next();
    match next {
        Some(Token::Num(i)) => Ok(visitor.num(i)),
        Some(Token::LParen) => {
            let e = visitor.paren(parse_expr(lexer, visitor)?);
            expect(&lexer.next(), &[Some(Token::RParen)], ")")?;
            Ok(e)
        },
        _ => Err("Error when parsing factor")
    }
}


//Visitor 1: Evaluator
struct Evaluator;
impl ExprVisitor for Evaluator {
    type Expr = i64;
    type Term = i64;
    type Factor = i64;
    fn num (&self, i: i64) -> i64 { 
        i 
    }
    fn mul (&self, l: i64, r: i64) -> i64 {
        l * r
    }
    fn div (&self, l: i64, r: i64) -> i64 {
        l / r
    }
    fn add (&self, l: i64, r: i64) -> i64 {
        l + r
    }
    fn sub (&self, l: i64, r: i64) -> i64 {
        l - r
    }
    fn paren (&self, i: i64) -> i64 { i }
    fn factor_term (&self, i: i64) -> i64 { i }
    fn term_expr (&self, i: i64) -> i64 { i }

}

//Visitor 2: Parse tree builder
#[derive(Debug)]
enum ExprNode {
    Add(Box<ExprNode>, Box<TermNode>),
    Sub(Box<ExprNode>, Box<TermNode>),
    Term(Box<TermNode>)
}

#[derive(Debug)]
enum TermNode {
    Mul(Box<TermNode>, Box<FactorNode>),
    Div(Box<TermNode>, Box<FactorNode>),
    Factor(Box<FactorNode>)
}

#[derive(Debug)]
enum FactorNode {
    Paren(Box<ExprNode>),
    Num(i64)
}

struct SyntaxTreeBuilder {}

impl ExprVisitor for SyntaxTreeBuilder {
    type Expr = Box<ExprNode>;
    type Term = Box<TermNode>;
    type Factor = Box<FactorNode>;

    fn add (&self, l: Box<ExprNode>, r: Box<TermNode>) -> Box<ExprNode> {
        Box::new(ExprNode::Add(l, r))
    }
    fn sub (&self, l: Box<ExprNode>, r: Box<TermNode>) -> Box<ExprNode> {
        Box::new(ExprNode::Sub(l, r))
    }
    fn term_expr (&self, t: Box<TermNode>) -> Box<ExprNode> {
        Box::new(ExprNode::Term(t))
    }
    fn mul (&self, l: Box<TermNode>, r: Box<FactorNode>) -> Box<TermNode> {
        Box::new(TermNode::Mul(l, r))
    }
    fn div (&self, l: Box<TermNode>, r: Box<FactorNode>) -> Box<TermNode> {
        Box::new(TermNode::Div(l, r))
    }
    fn factor_term (&self, f: Box<FactorNode>) -> Box<TermNode> {
        Box::new(TermNode::Factor(f))
    }
    fn paren (&self, e: Box<ExprNode>) -> Box<FactorNode> {
        Box::new(FactorNode::Paren(e))
    }
    fn num (&self, i: i64) -> Box<FactorNode> {
        Box::new(FactorNode::Num(i))
    }
}

fn parse<'a, V: ExprVisitor> (s: &str, visitor: &V) -> Result<<V as ExprVisitor>::Expr, &'a str> {
    parse_expr(&mut Lexer::new(s).filter(|t|*t!=Token::Ws).peekable(), visitor)
}

fn main() {
    let e1 = "2+3*(1-2)";
    let r1 = parse(e1, &Evaluator{}).unwrap();
    println!("{}={}", e1, r1);
        
    let r2 = parse(e1, &SyntaxTreeBuilder{}).unwrap();
    println!("{:?}", r2);

}
