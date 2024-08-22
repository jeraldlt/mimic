use thiserror::Error;

#[derive(Error, Debug)]
pub enum MimicError {
    #[error("The file {0} does not exist.")]
    FileDoesNotExist(String),

    #[error("Unknown register name {0}.")]
    UnknownRegister(String),

    #[error("Unknown mnemonic {0}.")]
    UnknownMnemonic(String),

    #[error("Instruction not yet implemented: {0}.")]
    UnimplementedInstruction(String),

    #[error("Out of bounds memory access at address {0}")]
    MemoryOutOfBounds(u32),
}
