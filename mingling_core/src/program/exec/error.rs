use crate::error::{ChainProcessError, ProgramPanic};
use std::fmt;

#[derive(Debug)]
pub enum ProgramExecuteError {
    DispatcherNotFound,
    RendererNotFound(String),
    Panic(ProgramPanic),
    Other(String),
}

impl fmt::Display for ProgramExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramExecuteError::DispatcherNotFound => write!(f, "No Dispatcher Found"),
            ProgramExecuteError::RendererNotFound(s) => {
                write!(f, "No Renderer (`{}`) Found", s)
            }
            ProgramExecuteError::Panic(p) => write!(f, "Panic: {:?}", p),
            ProgramExecuteError::Other(s) => write!(f, "Other error: {}", s),
        }
    }
}

impl std::error::Error for ProgramExecuteError {}

impl From<ProgramPanic> for ProgramExecuteError {
    fn from(value: ProgramPanic) -> Self {
        ProgramExecuteError::Panic(value)
    }
}

#[derive(Debug)]
pub enum ProgramInternalExecuteError {
    DispatcherNotFound,
    RendererNotFound(String),
    Other(String),
    IO(std::io::Error),
}

impl fmt::Display for ProgramInternalExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramInternalExecuteError::DispatcherNotFound => {
                write!(f, "No Dispatcher Found")
            }
            ProgramInternalExecuteError::RendererNotFound(s) => {
                write!(f, "No Renderer (`{}`) Found", s)
            }
            ProgramInternalExecuteError::Other(s) => write!(f, "Other error: {}", s),
            ProgramInternalExecuteError::IO(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for ProgramInternalExecuteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProgramInternalExecuteError::IO(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ProgramInternalExecuteError {
    fn from(e: std::io::Error) -> Self {
        ProgramInternalExecuteError::IO(e)
    }
}

impl From<ProgramInternalExecuteError> for ProgramExecuteError {
    fn from(value: ProgramInternalExecuteError) -> Self {
        match value {
            ProgramInternalExecuteError::DispatcherNotFound => {
                ProgramExecuteError::DispatcherNotFound
            }
            ProgramInternalExecuteError::RendererNotFound(s) => {
                ProgramExecuteError::RendererNotFound(s)
            }
            ProgramInternalExecuteError::Other(s) => ProgramExecuteError::Other(s),
            ProgramInternalExecuteError::IO(e) => ProgramExecuteError::Other(format!("{}", e)),
        }
    }
}

impl From<ChainProcessError> for ProgramInternalExecuteError {
    fn from(value: ChainProcessError) -> Self {
        match value {
            ChainProcessError::Other(s) => ProgramInternalExecuteError::Other(s),
            ChainProcessError::IO(error) => ProgramInternalExecuteError::IO(error),
        }
    }
}
