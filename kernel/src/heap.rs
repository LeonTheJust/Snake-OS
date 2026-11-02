use core::alloc::{Allocator, GlobalAlloc, Layout};
use core::ptr::NonNull;
use spin::Mutex;
use crate::serial::emergency_message;

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize
}

impl BumpAllocator {
    pub fn new(heap_start : usize, heap_end:usize) -> BumpAllocator {
        let next = heap_start;
        return BumpAllocator{heap_start, heap_end, next};
    }

    pub fn allocate(&mut self,layout: Layout) -> Option<usize> {
        let temp = align_up(self.next, layout.align());
        if temp > self.heap_end {
            emergency_message("Not enough memory to allocate");
            return None;
        }
        if layout.size() > self.heap_end - temp {
            emergency_message("Not enough memory to allocate");
            return None;
        }
        self.next = temp;
        let start_addr = self.next;
        self.next = self.next + layout.size();
        return Some(start_addr);
    }
}

pub fn align_up(address: usize, alignment: usize) -> usize {
    let mut temp = address / alignment;
    let remainder = address % alignment;
    if(remainder != 0){
        temp = (temp +1 )* alignment;
        return temp;}
    else{
        return address;
    }
}

pub struct BumpAllocatorWrapper{
    bump_allocator : Mutex<Option<BumpAllocator>>
}

impl BumpAllocatorWrapper {
    pub const fn new() -> BumpAllocatorWrapper {
        return BumpAllocatorWrapper{bump_allocator : Mutex::new(None)};
    }

    pub fn set_bump_allocator(&self, heap_start : usize, heap_end:usize) {
        let mut temp = self.bump_allocator.lock();
        /*match  temp{
            Some(bump_allocator) => {
                emergency_message("BumpAllocator has already been initialized");
            }
            None => {
                let mut bump_allocator = BumpAllocator::new (heap_start, heap_end);
                temp.replace(bump_allocator);
            }
        }*/
        if !temp.is_none(){
            emergency_message("BumpAllocator has already been initialized");
        }
        else{
            let bump_allocator = BumpAllocator::new (heap_start, heap_end);
            temp.replace(bump_allocator);
        }

    }
}

unsafe impl GlobalAlloc for BumpAllocatorWrapper {
    unsafe fn alloc(&self,layout: Layout) -> *mut u8 {
        let mut bump_allocator = self.bump_allocator.lock();
        if(bump_allocator.is_none()) {
            emergency_message("BumpAllocator not initialized");
            return 0 as *mut u8;
        }
        let res = bump_allocator.as_mut().unwrap().allocate(layout);
        return match res {
            None => {
                return 0 as *mut u8
            }
            Some(addr) => {
                addr as *mut u8
            }
        }
    }
    unsafe fn dealloc(&self,ptr: *mut u8,layout: Layout) {

    }
}
#[global_allocator]
pub static ALLOCATOR: BumpAllocatorWrapper = BumpAllocatorWrapper::new();