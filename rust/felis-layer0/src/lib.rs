pub struct File {
    pub items: Vec<FunDef>,
}

pub struct FunDef {
    pub name: Ident,
    pub params: Vec<Parameter>,
    pub ret_ty: TypeSignature,
    pub statements: Vec<Statement>,
}

pub struct Parameter {
    pub ident: Ident,
    pub ty: TypeSignature,
}
pub struct TypeSignature {
    pub ident: Ident,
    pub args: Vec<TypeSignatureArg>,
}

pub enum TypeSignatureArg {
    Ident(Ident),
    Number(Number),
}

pub enum Statement {
    Let(LetStatement),
    Const(ConstStatement),
    Return(ReturnStatement),
    Param(ParamStatement),
}

pub struct ParamStatement {
    pub ident: Ident,
    pub ty: TypeSignature,
}

pub struct LetStatement {
    pub ident: Ident,
    pub ty: TypeSignature,
    pub f: Ident,
    pub args: Vec<Ident>,
}

pub enum ConstValue {
    Number(Number),
}

pub struct ConstStatement {
    pub ident: Ident,
    pub ty: TypeSignature,
    pub const_value: ConstValue,
}

pub struct ReturnStatement {
    pub ident: Ident,
}

pub enum Token {
    Ident(Ident),
    Number(Number),
    LBrace(LBrace),
    RBrace(RBrace),
    LParen(LParen),
    RParen(RParen),
    Colon(Colon),
    Equal(Equal),
    Func(Func),
    Let(Let),
    Arrow(Arrow),
    Semicolon(Semicolon),
}

pub struct Ident {
    pub s: String,
}

pub struct Number {
    pub s: String,
}

pub struct LBrace;
pub struct RBrace;
pub struct LParen;
pub struct RParen;
pub struct Colon;
pub struct Equal;
pub struct Arrow;
pub struct Func;
pub struct Let;
pub struct Semicolon;

pub fn lex(s: &str) -> Vec<Token> {
    let cs: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut res = vec![];
    while i < cs.len() {
        while i < cs.len() && cs[i].is_whitespace() {
            i += 1;
        }
        if i >= cs.len() {
            break;
        }
        if cs[i].is_alphabetic() || cs[i] == '_' {
            let mut k = i;
            while k < cs.len() && (cs[k].is_alphanumeric() || cs[k] == '_') {
                k += 1;
            }
            let s: String = cs[i..k].iter().collect();
            i = k;
            if s == "let" {
                res.push(Token::Let(Let));
                continue;
            }
            if s == "fn" {
                res.push(Token::Func(Func));
                continue;
            }
            let ident = Ident {
                s,
            };
            res.push(Token::Ident(ident));
            continue;
        }
        if cs[i].is_numeric() {
            let mut k = i;
            while k < cs.len() && (cs[k].is_alphanumeric() || cs[k] == '_') {
                k += 1;
            }
            let s: String = cs[i..k].iter().collect();
            let number = Number {
                s,
            };
            res.push(Token::Number(number));
            i = k;
            continue;
        }
        if i + 1 < cs.len() && cs[i] == '-' && cs[i+1] == '>' {
            res.push(Token::Arrow(Arrow));
            i += 2;
            continue;
        }
        if cs[i] == '(' {
            res.push(Token::LParen(LParen));
            i += 1;
            continue;
        }
        if cs[i] == ')' {
            res.push(Token::RParen(RParen));
            i += 1;
            continue;
        }
        if cs[i] == '{' {
            res.push(Token::LBrace(LBrace));
            i += 1;
            continue;
        }
        if cs[i] == '}' {
            res.push(Token::RBrace(RBrace));
            i += 1;
            continue;
        }
        if cs[i] == ':' {
            res.push(Token::Colon(Colon));
            i += 1;
            continue;
        }
        if cs[i] == ';' {
            res.push(Token::Semicolon(Semicolon));
            i += 1;
            continue;
        }
        if cs[i] == '=' {
            res.push(Token::Equal(Equal));
            i += 1;
            continue;
        }
    }
    res
}
