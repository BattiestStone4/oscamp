//! Allocator algorithm in lab.

#![no_std]
#![allow(unused_variables)]

use allocator::{AllocError, BaseAllocator, ByteAllocator, AllocResult};
use core::ptr::NonNull;
use core::alloc::Layout;
use slab_allocator::Heap;

pub struct LabByteAllocator {
    inner: Option<Heap>,
}

impl LabByteAllocator {
    pub const fn new() -> Self {
        Self { inner: None }
    }

    fn inner_mut(&mut self) -> &mut Heap {
        self.inner.as_mut().unwrap()
    }

    fn inner(&self) -> &Heap {
        self.inner.as_ref().unwrap()
    }
}

impl BaseAllocator for LabByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        unsafe {
            self.inner = Some(Heap::new(start, size));
        }
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        unsafe {
            self.inner_mut().add_memory(start, size);
        }
        Ok(())
    }
}

impl ByteAllocator for LabByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        //log::info!("layout: {:?}", layout);
        self.inner_mut()
        .allocate(layout)
        .map_err(|_| AllocError::NoMemory)
    }
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        unsafe {
            self.inner_mut()
            .deallocate(pos.as_ptr() as usize, layout)
        }
    }
    fn total_bytes(&self) -> usize {
        self.inner().total_bytes()
    }
    fn used_bytes(&self) -> usize {
        self.inner().used_bytes()
    }
    fn available_bytes(&self) -> usize {
        self.inner().available_bytes()
    }
}
