use thiserror::Error;

#[derive(Error, Debug)]
pub enum JoltError {
    #[error("Create shape")]
    CreateShape,
    #[error("Too less subshape")]
    TooLessSubShape,
    #[error("Create body")]
    CreateBody,

    #[error("Engine update ({0})")]
    EngineUpdate(u32),
}

pub type JoltResult<T> = Result<T, JoltError>;
