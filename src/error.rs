use thiserror::Error;

#[derive(Error, Debug)]
pub enum JoltError {
    #[error("Create shape")]
    CreateShape,
    #[error("Create body")]
    CreateBody,
}

pub type JoltResult<T> = Result<T, JoltError>;
