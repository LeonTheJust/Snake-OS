use bootloader_api::info::{MemoryRegion, MemoryRegionKind, MemoryRegions};
use x86_64::PhysAddr;
use x86_64::structures::paging::{FrameAllocator, PhysFrame, Size4KiB};
use crate::serial;
use crate::serial::u64_to_str;

pub struct MemoryAllocator<'a> {
    memory_map : &'a MemoryRegions,
    current_region_index: usize,
    next_frame: u64,
}

impl MemoryAllocator<'_> {
    pub fn new(memory : &MemoryRegions) -> MemoryAllocator {
        let current_region_index = 0;
        let next_frame = 0;
        let memory_map = memory;
        return MemoryAllocator{memory_map, current_region_index, next_frame};
    }
}

unsafe impl FrameAllocator<Size4KiB> for MemoryAllocator<'_> {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        while(self.current_region_index + 1) <= self.memory_map.len() {
            let region = &self.memory_map[self.current_region_index];
            let temp : i64 = region.end as i64 - self.next_frame as i64;
            if(region.kind == MemoryRegionKind::Usable) {
                if temp >= 4096{
                    let address=  self.next_frame; //
                    let mut buffer = [0u8;20];
                    let physical_address = PhysAddr::new(address);
                    let frame = PhysFrame::<Size4KiB>::from_start_address(physical_address).expect("Frame address must be aligned");
                    self.next_frame += 4096;
                    serial::emergency_message("\nAddress:");
                    serial::emergency_message(u64_to_str(address as u64, &mut buffer));
                    serial::emergency_message("\n");
                    return Some(frame);
                }
            }
            if(!(region.kind == MemoryRegionKind::Usable) ||(temp < 4096 )) {
                self.current_region_index += 1;
                if(self.current_region_index < self.memory_map.len()) {
                    if(self.memory_map[self.current_region_index].start % 4096 == 0){
                        self.next_frame = self.memory_map[self.current_region_index].start;}
                    else{
                        self.next_frame = self.memory_map[self.current_region_index].start + 4096 - self.memory_map[self.current_region_index].start % 4096;
                    }
                    return self.allocate_frame();
                }
            }
        }
        return None;
    }
}