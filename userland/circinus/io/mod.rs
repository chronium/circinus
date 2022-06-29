pub mod stdin;
pub mod stdout;

pub use stdin::stdin;
pub use stdout::stdout;

pub const STDIN: usize = 0;
pub const STDOUT: usize = 1;
