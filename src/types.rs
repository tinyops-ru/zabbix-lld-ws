use crate::errors::OperationError;

pub type OperationResult<R> = Result<R, OperationError>;

pub type StringResult = Result<String, OperationError>;

pub type EmptyResult = Result<(), OperationError>;