use plex::parser;

use super::lexer::{Directive, Token};
use super::lexer::Token::*;
use crate::errors::Span;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Stmt {
    pub span: Span,
    pub statement: Stmt_,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Stmt_ {
    Section {
        section_directive: Expr,
        stmts: Box<Vec<Stmt>>,
    },
    DataDeclaration {
        label: Expr,
        type_directive: Expr,
        data: Vec<Expr>,
    },
    Instruction {
        mnemonic: Expr,
        args: Vec<Expr>,
    },
    LabelDeclaration {
        label: Expr,
    },
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Expr {
    pub span: Span,
    pub node: Expr_,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr_ {
    Ident(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    Register(String),
    TypeDirectiveExpression(Directive),
    SectionDirectiveExpression(Directive),
    Label(Box<Expr>),
}

parser! {
    fn parse_(Token, Span);

    (a, b) {
        Span {
            lo: a.lo,
            hi: b.hi,
            // source: a.source.clone(),
        }
    }

    Program: Vec<Stmt> {
        SectionList[s] => s,
    }

    SectionList: Vec<Stmt> {
        Section[s] => vec![s],
        SectionList[mut a] Section[s] => {
            a.push(s);
            a
        }
    }

    Section: Stmt {
        SectionDirectiveExpression[x] OptStatementList[stmts] => Stmt {
            span: span!(),
            statement: Stmt_::Section {
                section_directive: x,
                stmts: Box::new(stmts),
            }
        }
    }

    StatementList: Vec<Stmt> {
        Statement[s] => vec![s],
        StatementList[mut stmts] Statement[s] => {
            stmts.push(s);
            stmts
        }
    }

    OptStatementList: Vec<Stmt> {
        StatementList[stmts] => stmts,
        => vec![],
    }

    Statement: Stmt {
        DataDeclaration[s] => s,
        Instruction[s] => s,
        LabelDeclaration[s] => s,
    }


    DataDeclaration: Stmt {
        Label[x] TypeDirectiveExpression[t] PrimaryExpressionList[exprs] => Stmt {
            span: span!(),
            statement: Stmt_::DataDeclaration {
                label: x,
                type_directive: t,
                data: exprs,
            }
        }
    }

    Instruction: Stmt {
        Syscall => Stmt {
            span: span!(),
            statement: Stmt_::Instruction {
                mnemonic: Expr {
                    span: span!(),
                    node: Expr_::Ident("syscall".to_owned()),
                },
                args: vec![],
            }
        },
        Identifier[x] ArgumentList[args] => Stmt {
            span: span!(),
            statement: Stmt_::Instruction {
                mnemonic: x,
                args: args,
            }
        }
    }

    LabelDeclaration: Stmt {
        Label[x] => Stmt {
            span: span!(),
            statement: Stmt_::LabelDeclaration {
                label: x,
            }
        }
    }

    Label: Expr {
        Identifier[x] Colon => Expr {
            span: span!(),
            node: Expr_::Label(Box::new(x)),
        }
    }

    SectionDirectiveExpression: Expr {
        SectionDirective(d) => Expr {
            span: span!(),
            node: Expr_::SectionDirectiveExpression(d),
        }
    }

    TypeDirectiveExpression: Expr {
        TypeDirective(d) => Expr {
            span: span!(),
            node: Expr_::TypeDirectiveExpression(d),
        }
    }


    PrimaryExpressionList: Vec<Expr> {
        PrimaryExpression[x] => vec![x],
        PrimaryExpressionList[mut a] Comma PrimaryExpression[x] => {
            a.push(x);
            a
        }
    }


    ArgumentList: Vec<Expr> {
        Argument[x] => vec![x],
        ArgumentList[mut a] Comma Argument[x] => {
            a.push(x);
            a
        }
    }

    // OptArgumentList: Vec<Expr> {
    //     ArgumentList[args] => args,
    //     => vec![],

    Argument: Expr {
        RegisterExpression[x] => x,
        PrimaryExpression[x] => x,
    }

    RegisterExpression: Expr {
        Register(s) => Expr {
            span: span!(),
            node: Expr_::Register(s),
        }
    }

    PrimaryExpression: Expr {
        Literal[x] => x,
        Identifier[x] => x,
    }

    Literal: Expr {
        IntLiteral[x] => x,
        FloatLiteral[x] => x,
        StringLiteral[x] => x,
    }

    IntLiteral: Expr {
        Integer(i) => Expr {
            span: span!(),
            node: Expr_::IntLiteral(i),
        }
    }

    FloatLiteral: Expr {
        Float(f) => Expr {
            span: span!(),
            node: Expr_::FloatLiteral(f),
        }
    }

    StringLiteral: Expr {
        Str(s) => Expr {
            span: span!(),
            node: Expr_::StringLiteral(s),
        }
    }

    Identifier: Expr {
        Ident(s) => Expr {
            span: span!(),
            node: Expr_::Ident(s),
        }
    }
}


pub fn parse<I: Iterator<Item = (Token, Span)>>(i: I) -> Result<Vec<Stmt>, (Option<(Token, Span)>, &'static str)> {
    parse_(i)
}
