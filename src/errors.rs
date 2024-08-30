use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term;
use codespan_reporting::term::Config;
use codespan_reporting::term::termcolor::{StandardStream, ColorChoice};


use std::fmt;
use std::path::PathBuf;
use std::ops::Range;


#[derive(Debug, Clone, Copy)]
pub struct Span {
    // pub source: SimpleFile<String, String>,
    pub lo: usize,
    pub hi: usize,
}

impl Span {
    pub fn range(&self) -> Range<usize> {
        self.lo..self.hi
    }
}

#[derive(Debug)]
pub struct MimicError {
    pub span: Option<Span>,
    pub source: Option<SimpleFile<String, String>>,
    pub ty: MimicErrorType,
}

#[derive(Debug)]
pub enum MimicErrorType {
    FileDoesNotExist {
        filename: PathBuf,
    },

    UnknownToken {
        token: String,
    },

    UnknownRegister {
        register_name: String,
    },

    IncorrectArgument {},

    IncorrectArgumentType {},

    UnknownMnemonic {
        mnemonic: String,
    },

    UnimplementedInstruction {
        mnemonic: String,
    },

    MemoryOutOfBounds {
        address: usize,
    },

}

impl MimicError {
    pub fn msg(&self) -> String {
        match &self.ty {
            MimicErrorType::FileDoesNotExist{filename} => {
                format!("File path [{:?}] does not exist", filename)
            },

            MimicErrorType::UnknownToken { token } => {
                format!("Unknown token [{}]", token)
            },

            MimicErrorType::UnknownRegister { register_name } => {
                format!("Unknown register name [{}]", register_name)
            },

            MimicErrorType::IncorrectArgument { } => {
                format!("Incorrect register")
            },

            MimicErrorType::IncorrectArgumentType { } => {
                format!("Incorrect register type")
            },

            MimicErrorType::UnknownMnemonic { mnemonic } => {
                format!("The mnemonic [{}] is either incorrect or for an unsupported instruction", mnemonic)
            },

            MimicErrorType::UnimplementedInstruction { mnemonic } => {
                format!("Unimplemented instruction [{}]", mnemonic)
            },

            MimicErrorType::MemoryOutOfBounds { address } => {
                format!("Out of bounds memory access at address {}", address)
            },
        }
    }

    pub fn emit(&self) {
        if let Some(f) = &self.source {
            let e = match &self.ty {
                MimicErrorType::UnknownToken { token } => {
                    Diagnostic::error()
                        .with_message(self.msg())
                        .with_labels(vec![
                            Label::primary((), self.span.as_ref().unwrap().range())
                                .with_message(format!("Token {}", token.clone())),
                        ])
                },
                
                _ => Diagnostic::error()
                        .with_message(self.msg()),

            };

            let writer = StandardStream::stderr(ColorChoice::Always);
            term::emit(&mut writer.lock(), &Config{ ..Default::default()}, f, &e).expect("Unable to print error message");
        }

    }

    // pub fn report(&self) -> Diagnostic<()> {
    //     return if let Some(span) = self.span {
    //         Diagnostic::error()
    //             .with_message(self.msg())
    //             .with_labels
    //     }
    // }
}


impl fmt::Display for MimicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg())
    }
        
}
