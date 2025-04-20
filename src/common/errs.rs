use std::result;

use thiserror::Error;

/// enum containing all possible errors that
/// this crate could ever encounter.

#[derive(Error, Debug)]
pub enum Error {
}

/// wrapper around std::result::Result, for easier
/// error propagation and to avoid having to repeat
/// the error type in every function signature.

pub type Result<T> = result::Result<T, Error>;

/// wrapper around Result<()>, for even better developer
/// experience when dealing with functions that might fail,
/// but don't return any value.

pub type ResultVoid = result::Result<(), Error>;
