use anyhow;

pub type OperationResult<R> = anyhow::Result<R>;
pub type OptionalResult<R> = anyhow::Result<Option<R>>;

pub type EmptyResult = anyhow::Result<()>;