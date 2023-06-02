use anyhow;

pub type OperationResult<R> = anyhow::Result<R>;

pub type EmptyResult = anyhow::Result<()>;