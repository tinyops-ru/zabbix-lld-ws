pub mod errors {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum OperationError {
        #[error("Unknown error")]
        Error,

        #[error(transparent)]
        IOError(#[from] std::io::Error)
    }
}
