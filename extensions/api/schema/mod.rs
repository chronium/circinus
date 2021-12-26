pub enum SchemaError {}

pub type Result<T> = core::result::Result<T, SchemaError>;

pub mod fs;
pub mod posix;
pub mod unix;
