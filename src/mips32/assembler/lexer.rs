use plex::lexer;
use codespan_reporting::files::SimpleFile;

use crate::errors::{MimicError, MimicErrorType, Span};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Token {
    Whitespace,
    Newline,
    Comment,
    Ident(String),
    Register(String),
    Integer(i64),
    Float(f64),
    Str(String),
    Period,
    Colon,
    Comma,
    DollarSign,
    Unknown(String),
    SectionDirective(Directive),
    TypeDirective(Directive),
    Syscall,

}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Directive {
    Align,
    Ascii,
    Asciiz,
    Byte,
    Data,
    Double,
    Extern,
    Float,
    Globl,
    Half,
    Kdata,
    Ktext,
    Space,
    Text,
    Word,
}

lexer! {
    fn next_token(text: 'a) -> Token;

    r#"[ \t\r]+"# => Token::Whitespace,
    r#"\n"# => Token::Newline,
    r#"#[^\n]*"# => Token::Comment,

    r#"\"(\\.|[^\\"\n])*\""# => Token::Str(text.to_owned()),
    r#"[0-9]+"# => {
        if let Ok(i) = text.parse() {
            Token::Integer(i)
        } else {
            panic!("integer {} is out of range", text)
        }
    },
    r#"0[xX][0-9a-fA-F]+"# => {
        if let Ok(i) = u32::from_str_radix(&text[2..], 16) {
            Token::Integer(i as i64)
        } else {
            panic!("integer {} is out of range", text)
        }
    },

    r#"\.text"# => Token::SectionDirective(Directive::Text),
    r#"\.data"# => Token::SectionDirective(Directive::Data),
    r#"\.asciiz"# => Token::TypeDirective(Directive::Asciiz),

    r#"syscall"# => Token::Syscall,

    r#"\$[a-zA-Z0-9]+"# => Token::Register(text.to_owned()),

    r#":"# => Token::Colon,
    // r#"\."# => Token::Period,
    r#","# => Token::Comma,
    // r#"\$"# => Token::DollarSign,

    r#"[a-zA-Z_][a-zA-Z0-9_]*"# => Token::Ident(text.to_owned()),


    r#"."# => Token::Unknown(text.to_owned()),
    
}


#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
    source: SimpleFile<String, String>,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str, source: SimpleFile<String, String>) -> Lexer<'a> {
        Lexer {
            original: s,
            remaining: s,
            source,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Span);
    fn next(&mut self) -> Option<(Token, Span)> {
        loop {
            let (tok, span) = if let Some((tok, new_remaining)) = next_token(self.remaining) {
                let lo = self.original.len() - self.remaining.len();
                let hi = self.original.len() - new_remaining.len();
                self.remaining = new_remaining;
                // (tok, Span {source: self.source.clone(), lo, hi})
                (tok, Span {lo, hi})
            } else {
                return None;
            };

            match tok {
                Token::Whitespace => continue,
                Token::Comment => continue,
                Token::Newline => continue,
                Token::Unknown(t) => MimicError {
                    span: Some(span),
                    source: Some(self.source.clone()),
                    ty: MimicErrorType::UnknownToken{token: t},
                }.emit(),

                _ => return Some((tok, span)),
            }
        }
    }
}


