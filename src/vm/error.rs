use std::io;

use crate::decoder::DecodeError;

#[derive(Debug, Fail)]
pub enum RuntimeError {
    #[fail(display = "Uncaught TypeError.")]
    TypeError,

    #[fail(display = "LabelError.")]
    LabelError,

    #[fail(display = "Stack Pop Error: Failed to pop from stack.")]
    StackPopError,

    #[fail(display = "Type Mismatch Operation Error.")]
    TypeMismatchOperationError,

    #[fail(display = "out of bounds memory access")]
    OutOfBoundsMemoryAccessError,

    #[fail(display = "integer divide by zero")]
    DivisionByZeroError,

    #[fail(display = "integer overflow")]
    DivisionOverflowError,

    #[fail(display = "integer overflow")]
    IntegerOverflowError,

    #[fail(display = "indirect call type mismatch")]
    IndirectCallTypeMismatchError,

    #[fail(display = "invalid conversion to integer")]
    UnrepresentableIntegerError,

    #[fail(display = "undefinedMemoryError: please define and load memory")]
    UndefinedMemoryError,

    #[fail(display = "undefinedGlobalError: please define global")]
    UndefinedGlobalError,

    #[fail(display = "undefinedTableError: please define table")]
    UndefinedTableError,

    #[fail(display = "undefinedFunctionError: please define function")]
    UndefinedFunctionError,

    #[fail(display = "undefinedExportError: please define export")]
    UndefinedExportError,

    #[fail(display = "undefined element")]
    UndefinedElementError,

    #[fail(display = "uninitialized element")]
    UnInitializedElementError,

    #[fail(display = "Some I/O Error: {:?}", error)]
    IOError { error: io::Error },

    #[fail(display = "Failed to decode wasm file. Please specify valid wasm file")]
    DecodeError { error: DecodeError },
}

impl From<io::Error> for RuntimeError {
    fn from(error: io::Error) -> Self {
        RuntimeError::IOError { error }
    }
}

impl From<DecodeError> for RuntimeError {
    fn from(error: DecodeError) -> Self {
        RuntimeError::DecodeError { error }
    }
}
