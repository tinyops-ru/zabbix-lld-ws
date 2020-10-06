pub mod types {
    use crate::errors::errors::OperationError;

    pub type StringResult = Result<String, OperationError>;
}
