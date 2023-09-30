use core::marker::PhantomData;

use alloc::sync::Arc;
use spin::mutex::Mutex;

pub struct SyncPointer<T> {
	ptr: Arc<Mutex<usize>>,
	t: PhantomData<T>,
}

impl<T> Clone for SyncPointer<T> {
	fn clone(&self) -> Self {
		Self {
			ptr: self.ptr.clone(),
			t: self.t.clone(),
		}
	}
}

impl<T> SyncPointer<T> {
	pub fn new(t: &T) -> Self {
		Self {
			ptr: Arc::new(Mutex::new(t as *const _ as usize)),
			t: PhantomData,
		}
	}

	pub fn as_ptr(&self) -> *mut T {
		let ptr = self.ptr.lock();
		*ptr as *mut T
	}

	pub fn as_ref(&self) -> &mut T {
		let ptr = self.ptr.lock();
		unsafe { &mut *(*ptr as *mut T) }
	}
}

unsafe impl<T> Sync for SyncPointer<T> {}
