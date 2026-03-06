use thiserror::Error;

/// Root error type for the COBOL runtime.
#[derive(Error, Debug)]
pub enum CobolError {
    #[error("File I/O error: {0}")]
    File(#[from] FileError),

    #[error("Sort error: {0}")]
    Sort(#[from] SortError),

    #[error("Arithmetic error: {0}")]
    Arith(#[from] ArithError),

    #[error("Call error: {0}")]
    Call(#[from] CallError),

    #[error("Configuration error: {0}")]
    Config(String),
}

/// File I/O errors mapped to COBOL FILE STATUS codes.
#[derive(Error, Debug)]
pub enum FileError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("File not found: {0}")]
    NotFound(String),

    #[error("File already open: {0}")]
    AlreadyOpen(String),

    #[error("File not open: {0}")]
    NotOpen(String),

    #[error("Duplicate key")]
    DuplicateKey,

    #[error("Record not found")]
    RecordNotFound,

    #[error("End of file")]
    EndOfFile,

    #[error("Sequence error")]
    SequenceError,
}

/// SORT/MERGE operation errors.
#[derive(Error, Debug)]
pub enum SortError {
    #[error("I/O error during sort: {0}")]
    Io(#[from] std::io::Error),

    #[error("Insufficient work space for sort")]
    InsufficientWorkSpace,

    #[error("Nested SORT not allowed")]
    NestedSort,
}

/// Arithmetic operation errors.
#[derive(Error, Debug)]
pub enum ArithError {
    #[error("Division by zero")]
    DivisionByZero,

    #[error("Exponentiation overflow")]
    ExponentiationOverflow,
}

/// CALL/CANCEL operation errors.
#[derive(Error, Debug)]
pub enum CallError {
    #[error("program not found: {0}")]
    ProgramNotFound(String),

    #[error("call failed: {0}")]
    CallFailed(String),
}
