use std::result;

use thiserror::Error;

/// enum containing all possible errors that
/// this crate could ever encounter.

#[derive(Error, Debug)]
pub enum Error {
    /// error that occurs when the `env_logger` crate
    /// fails to properly initialize itself.

    #[error("failed to initialize/set env_logger. {ctx}", ctx = source)]
    LogInit {
        #[from] #[source]
        source: log::SetLoggerError,
    },

    /// error that occurs when the `glfw` crate
    /// fails to properly initialize itself.
    
    #[error("failed to initialize glfw. {ctx}", ctx = source)]
    GlfwInit {
        #[from] #[source]
        source: glfw::InitError,
    },

    /// error that occurs when the `glfw` crate
    /// fails to create a window.
    
    #[error("failed to create glfw window.")]
    GlfwWindow,

    /// error that occurs when the `gl` crate 
    /// fails to properly initialize itself or
    /// load the opengl functions.
    
    #[error("failed to initialize opengl. {ctx}", ctx = context)]
    GlInit {
        context: String
    },
}

/// wrapper around std::result::Result, for easier
/// error propagation and to avoid having to repeat
/// the error type in every function signature.

pub type Result<T> = result::Result<T, Error>;

/// wrapper around Result<()>, for even better developer
/// experience when dealing with functions that might fail,
/// but don't return any value.

pub type ResultVoid = result::Result<(), Error>;
