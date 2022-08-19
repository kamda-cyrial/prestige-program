use solana_program::{program_error::ProgramError, msg};
use thiserror::Error;
use num_derive::FromPrimitive;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum PrestigeError{
    #[error("Dissimilarity between assert pubkeys")]
    AddressMismatch,

    #[error("Unknown Error")]
    UnknownError,

}


impl From<PrestigeError> for ProgramError {
    fn from(e: PrestigeError) -> Self {
        e.to_string();
        ProgramError::Custom(e as u32)
    }
}

impl PrestigeError{
    pub fn log(self, keyword:Option<&str>)->ProgramError{
        match self {
            Self::AddressMismatch => {msg!("Error:  Dissimilarity in the provided keypairs");}

            _ =>  if let Some(keyword) = keyword{msg!("Error: keyword{:?} Error not labeled!!", keyword)}
            
        }
        ProgramError::from(self)
    }
}