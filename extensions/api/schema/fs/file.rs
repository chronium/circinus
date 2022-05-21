use core::fmt::Debug;

use alloc::sync::Arc;

use crate::{io, Result};

pub trait File: Debug + Send + Sync {
	fn open(&self, _options: &io::OpenOptions) -> Result<Arc<dyn File>>;

	// fn size(&self) -> usize;
}
