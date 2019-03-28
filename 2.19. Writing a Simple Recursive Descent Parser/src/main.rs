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
        static tokens : [(&str, &(Fn(&str)->Result<Token, &str> + Sync)); 8] = [
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
            static ref r : regex::Regex = regex::Regex::new(&tokens.iter().map(|( pat, _)|format!("(^{})", pat)).collect::<Vec<_>>().join("|")).unwrap();
        }

        if let Some(captures) = r.captures(&self.input[self.cursor..]) {
            for (i, (_, conv)) in tokens.iter().enumerate() {
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
    type T;
    fn num (&self, i: i64) -> Self::T;
    fn mul (&self, l: Self::T, r: Self::T) -> Self::T;
    fn div (&self, l: Self::T, r: Self::T) -> Self::T;
    fn add (&self, l: Self::T, r: Self::T) -> Self::T;
    fn sub (&self, l: Self::T, r: Self::T) -> Self::T;
}

fn expect<'a, T:PartialEq>(next: &T, domain: &[T], err: &'a str) -> Result<(), &'a str> {
    if let Some(_) = domain.iter().find(|x|*x==next) {
        Ok(())
    } else {
        Err(err)
    }
}

fn parseExpr<'a, V:ExprVisitor, L:Iterator<Item=Token>> (lexer: &mut Peekable<L>, visitor: &V) -> Result<<V as ExprVisitor>::T, &'a str> {
    // expression ::= term { ('+'|'-') term }*"
    let mut left = parseTerm(lexer, visitor)?;
    loop{
        if lexer.peek()==Some(&Token::Add) || lexer.peek()==Some(&Token::Sub) {
            let next = lexer.next().unwrap();
            let right = parseTerm(lexer, visitor)?;
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

fn parseTerm<'a, V:ExprVisitor, L:Iterator<Item=Token>> (lexer: &mut Peekable<L>, visitor: &V) -> Result<<V as ExprVisitor>::T, &'a str> {
    // term ::= factor { ('*'|'/') factor }*
    let mut left = parseFactor(lexer, visitor)?;
    
    loop {
        if lexer.peek()==Some(&Token::Mul) || lexer.peek()==Some(&Token::Div) {
            let next = lexer.next().unwrap();
            let right = parseFactor(lexer, visitor)?;
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

fn parseFactor<'a, V:ExprVisitor, L:Iterator<Item=Token>> (lexer: &mut Peekable<L>, visitor: &V) -> Result<<V as ExprVisitor>::T, &'a str> {
    // factor ::= NUM | ( expr )
    let next = lexer.next();
    match next {
        Some(Token::Num(i)) => Ok(visitor.num(i)),
        Some(Token::LParen) => {
            let e = parseExpr(lexer, visitor)?;
            expect(&lexer.next(), &[Some(Token::RParen)], ")")?;
            Ok(e)
        },
        _ => Err("Error when parsing factor")
    }
}


//Visitor 1: Evaluator
struct Evaluator;
impl ExprVisitor for Evaluator {
    type T = i64;
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

}

//Visitor 2: Grammar tree builder
#[derive(Debug)]
enum Expr {
    Add(Box<Expr>, Box<Term>),
    Sub(Box<Expr>, Box<Term>),
    Term(Box<Term>)
}

#[derive(Debug)]
enum Term {
    Mul(Box<Term>, Box<Factor>),
    Div(Box<Term>, Box<Factor>),
    Factor(Box<Factor>)
}

#[derive(Debug)]
enum Factor {
    Paren(Box<Expr>),
    Num(i64)
}


fn main() {
    let e1 = "2+3*(1-2)";
    let r1 = parseExpr(&mut Lexer::new(e1).filter(|t|*t!=Token::Ws).peekable(), &Evaluator{}).unwrap();
    println!("{}={}", e1, r1)

}
