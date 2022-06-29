pub fn read_secure_random(
	buf: api::user_buffer::UserBufferMut<'_>,
) -> api::Result<usize> {
	// TODO: Implement arch-agnostic CRNG which does not fully depends on
	// RDRAND.

	api::user_buffer::UserBufWriter::from(buf).write_with(|slice| {
		let valid = unsafe { x86::random::rdrand_slice(slice) };
		if valid {
			Ok(slice.len())
		} else {
			warn_once!("RDRAND returned invalid data");
			Ok(0)
		}
	})
}

pub fn read_insecure_random(
	buf: api::user_buffer::UserBufferMut<'_>,
) -> api::Result<usize> {
	// TODO:
	read_secure_random(buf)
}
