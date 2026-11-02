use bootloader_api::info::{MemoryRegion, MemoryRegionKind, MemoryRegions};
use x86_64::{PhysAddr, VirtAddr};
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB, Translate};
use crate::serial;
use crate::serial::{emergency_message, u64_to_str};
use x86_64::structures::paging::PageTableFlags;

pub struct MemoryAllocator<'a> {
    memory_map : &'a MemoryRegions,
    current_region_index: usize,
    next_frame: u64,
}

impl MemoryAllocator<'static> {
    pub fn new(memory : &MemoryRegions) -> MemoryAllocator {
        let current_region_index = 0;
        let next_frame = 0;
        let memory_map = memory;
        return MemoryAllocator{memory_map, current_region_index, next_frame};
    }
}

/*struct OffsetPageTable<'a>{
    level_4_table : &'a mut PageTable,
    physical_offset : VirtAddr,
}

impl<'a> OffsetPageTable<'a> {
    unsafe fn new( physical_offset : VirtAddr) -> OffsetPageTable<'a> {
        let virtual_addr = level_4_table_reference(physical_offset);
        let level_4_table: *mut PageTable = virtual_addr.as_mut_ptr();
        return OffsetPageTable{level_4_table, physical_offset};
    }
}*/

pub unsafe fn init_level_4_table(physical_offset : VirtAddr) -> OffsetPageTable <'static> {
    let offset = physical_offset.as_u64();
    let (level_4_frame, _) = Cr3::read();
    let mut buffer = [0u8;20];
    let physical_address = level_4_frame.start_address();
    emergency_message("\nVirtual Memory Address:");
    emergency_message(u64_to_str(physical_address.as_u64() + offset,&mut buffer));
    let virtual_address =VirtAddr::new(physical_address.as_u64() + offset);
    let page_table_ptr: *mut PageTable = virtual_address.as_mut_ptr();
    let level_4_table: &mut PageTable = &mut *page_table_ptr; //already inside an unsafe block
    let offset_page_table = OffsetPageTable::new(level_4_table,VirtAddr::new(offset));
    return offset_page_table;
}

pub unsafe fn map_page_and_frame(offset_page_table: &mut OffsetPageTable, memory_allocator: &mut MemoryAllocator, virtual_page : Page<Size4KiB>) {
    let some_frame = memory_allocator.allocate_frame();
    if( some_frame != None) {
        let frame = some_frame.unwrap();
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        let res =offset_page_table.map_to(virtual_page,frame,flags,memory_allocator);
        match res {
            Ok(flush) =>{
                flush.flush();
            }
            Err(_) => panic!("Failed to map page, what am i doing maaaaannnnnnnnn"),
        }
    }
    else{
        panic!("No frame input!")
    }
}

pub fn find_free_range(number_of_pages :u64, offset_page_table:&mut OffsetPageTable, virtual_page : Page<Size4KiB>) -> Option<Page<Size4KiB>>{
    if number_of_pages == 0 || number_of_pages > 256 { //1MiB limit for now, will increase later (potentially 128MiB)
        return None;
    }
    let mut total_page_count = 0;
    let mut count: u64 = 0;
    let mut page = virtual_page;
    loop {
        let current = offset_page_table.translate_addr(page.start_address());
        match current {
            Some(_) => {
                count = 0;
            }
            None => {
                count = count + 1;
            }
        }
        page = Page::<Size4KiB>::containing_address(page.start_address() + 4096);
        if (count == number_of_pages) {
            break;
        }
        total_page_count += 1;
        if total_page_count > 256 { //1MiB limit for the whole search! will increase in the future
            return None
        }
    }
    let new_page = Page::<Size4KiB>::containing_address(VirtAddr::new(page.start_address().as_u64() - count * 4096));
    return Some(new_page);
}
pub unsafe fn map_multiple_pages(number_of_pages : u64, offset_page_table: &mut OffsetPageTable, memory_allocator : &mut MemoryAllocator, virtual_page : Page<Size4KiB>) -> Page<Size4KiB> {
    let first_page_opt = find_free_range(number_of_pages, offset_page_table, virtual_page);
    if(first_page_opt == None) {
        panic!("Failed to map multiple pages, not enough pages found!")
    }
    let mut first_page = first_page_opt.unwrap();
    for i in 0..number_of_pages {
        unsafe{
            map_page_and_frame(offset_page_table, memory_allocator, first_page);
            first_page = Page::<Size4KiB>::containing_address(first_page.start_address() + 4096);
        }
    }
    return first_page_opt.unwrap();
}

unsafe impl FrameAllocator<Size4KiB> for MemoryAllocator<'_> {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        while(self.current_region_index + 1) <= self.memory_map.len() {
            let mut buffer = [0u8;20];
            let region = &self.memory_map[self.current_region_index];
            let test = region.start;
            emergency_message("Start:");
            emergency_message(u64_to_str(test, &mut buffer));
            emergency_message("\n");
            emergency_message("End:");
            emergency_message(u64_to_str(region.end, &mut buffer));
            emergency_message("\n");
            if(region.kind == MemoryRegionKind::Usable){
                emergency_message("usable\n");
            }
            let temp : i64 = region.end as i64 - self.next_frame as i64;
            if(region.kind == MemoryRegionKind::Usable) {
                if temp >= 4096{
                    let address=  self.next_frame; //
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