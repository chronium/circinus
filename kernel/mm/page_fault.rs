use core::{cmp::min, slice};

use api::{
	arch::PAGE_SIZE,
	io::OpenOptions,
	mm::{alloc_pages, AllocPageFlags},
};
use environment::{address::UserVAddr, arch::PageFaultReason};
use utils::alignment::align_down;

use crate::process::current_process;

use super::vm::VmAreaType;

pub fn handle_page_fault(unaligned_vaddr: Option<UserVAddr>, ip: usize, _reason: PageFaultReason) {
	let unaligned_vaddr = match unaligned_vaddr {
		Some(unaligned_vaddr) => unaligned_vaddr,
		None => {
			debug_warn!(
				"null pointer access (vaddr={:x} ip={:x} reason={:?}), killing the current \
				 process...",
				unaligned_vaddr
					.unwrap_or(unsafe {
						environment::address::UserVAddr::new_unchecked(usize::MAX)
					})
					.value(),
				ip,
				_reason
			);
			// TODO: Process::exit_by_signal(signal::SIGSEGV);
			loop {}
		}
	};

	let current = current_process();
	let aligned_vaddr = match UserVAddr::new_nonnull(align_down(unaligned_vaddr.value(), PAGE_SIZE))
	{
		Ok(uaddr) => uaddr,
		_ => {
			debug_warn!(
				"invalid memory access at {} (ip={:x}), killing the current process...",
				unaligned_vaddr,
				ip
			);
			// TODO: Process::exit_by_signal(SIGSEGV);
			loop {}
		}
	};

	// Look for the associated vma area.
	let vm_ref = current.vm();
	let mut vm = vm_ref.as_ref().unwrap().lock();
	let vma = match vm
		.vm_areas()
		.iter()
		.find(|vma| vma.contains(unaligned_vaddr))
	{
		Some(vma) => vma,
		None => {
			debug_warn!(
				"no VMAs for address {}, killing the current process...",
				unaligned_vaddr
			);
			drop(vm);
			drop(vm_ref);
			// TODO: Process::exit_by_signal(SIGSEGV);
			loop {}
		}
	};

	// Allocate and fill the page.
	let paddr = alloc_pages(1, AllocPageFlags::USER).expect("failed to allocate an anonymous page");
	unsafe {
		paddr.as_mut_ptr::<u8>().write_bytes(0, PAGE_SIZE);
	}
	match vma.area_type() {
		VmAreaType::Anonymous => { // The page is already filled with zeros. Nothing to do.
		}
		VmAreaType::File {
			file,
			offset,
			file_size,
		} => {
			let buf = unsafe { slice::from_raw_parts_mut(paddr.as_mut_ptr(), PAGE_SIZE) };
			let offset_in_page;
			let offset_in_file;
			let copy_len;
			if aligned_vaddr < vma.start() {
				offset_in_page = unaligned_vaddr.value() % PAGE_SIZE;
				offset_in_file = *offset;
				copy_len = min(*file_size, PAGE_SIZE - offset_in_page);
			} else {
				let offset_in_vma = vma.offset_in_vma(aligned_vaddr);
				offset_in_page = 0;
				if offset_in_vma >= *file_size {
					offset_in_file = 0;
					copy_len = 0;
				} else {
					offset_in_file = offset + offset_in_vma;
					copy_len = min(*file_size - offset_in_vma, PAGE_SIZE);
				}
			}

			if copy_len > 0 {
				file.read(
					offset_in_file,
					(&mut buf[offset_in_page..(offset_in_page + copy_len)]).into(),
					&OpenOptions::readwrite(),
				)
				.expect("failed to read file");
			}
		}
	}

	// Map the page in the page table.
	vm.page_table_mut().map_user_page(aligned_vaddr, paddr);
}
