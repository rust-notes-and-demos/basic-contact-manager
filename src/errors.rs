use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProgramError {
    #[error("encountered an issue while trying to display table")]
    PrintTableError,

    #[error("couldn't find any entry")]
    EntryNotFoundError,

    #[error("some error occurred while trying to access data")]
    DataAccessError,

    #[error("couldn't write to file")]
    WriteError,

    #[error("Custom Error: {val:?}")]
    CustomError { val: String },
}