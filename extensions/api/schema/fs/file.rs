use core::fmt::Debug;

use alloc::sync::Arc;

use crate::io::{self, OpenOptions};

pub trait File: Debug + Send + Sync {
	fn open(&self, options: &OpenOptions) -> io::Result<Arc<dyn File>>;
}
