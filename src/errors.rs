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

impl PartialEq for ProgramError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ProgramError::PrintTableError, ProgramError::PrintTableError) => true,
            (ProgramError::EntryNotFoundError, ProgramError::EntryNotFoundError) => true,
            (ProgramError::DataAccessError, ProgramError::DataAccessError) => true,
            (ProgramError::WriteError, ProgramError::WriteError) => true,
            (ProgramError::CustomError { val: ref v1 }, ProgramError::CustomError { val: ref v2 }) => v1 == v2,
            _ => false,
        }
    }
}