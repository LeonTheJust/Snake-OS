use spin::Once;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;
use x86_64::registers::segmentation::{CS, Segment, SS};
use x86_64::instructions::tables::load_tss;
const STACK_SIZE: usize = 4096;

#[repr(C, align(16))]
struct Stack {
    bytes: [u8; STACK_SIZE],
}

static mut STACK_BUFFER: Stack = Stack {
    bytes: [0; STACK_SIZE],
};

pub struct TSS{
    task_state_segment: TaskStateSegment,
}

impl TSS{
    pub fn new() -> Self{
        let mut task_state_segment = TaskStateSegment::new();
        unsafe{ let mut temp_add = core::ptr::addr_of_mut!(STACK_BUFFER.bytes).cast::<u8>();
                temp_add = temp_add.add(STACK_SIZE);
                task_state_segment.interrupt_stack_table[0] = VirtAddr::from_ptr(temp_add);
        }


        //task_state_segment.interrupt_stack_table =
        return TSS{task_state_segment};
    }
}

static TASK_STATE_SEGMENT: Once<TSS> = Once::new();

pub fn init(){
    let gdt : &'static GDT = GDT_INSTANCE.call_once(GDT::new);
    gdt.global_descriptor_table.load();
    unsafe {
        <CS as Segment>::set_reg(gdt.code_selector);
        load_tss(gdt.tss_selector);
    }
    unsafe {
        <SS as Segment>::set_reg(gdt.data_selector);
    }
}
pub struct GDT{
    global_descriptor_table: GlobalDescriptorTable,
    tss_selector : SegmentSelector,
    code_selector : SegmentSelector,
    data_selector : SegmentSelector,
}

impl GDT{
    pub fn new() -> Self{
        let mut global_descriptor_table = GlobalDescriptorTable::new();
        let code_selector1 = Descriptor::kernel_code_segment();
        let tss1 = TASK_STATE_SEGMENT.call_once(TSS::new);
        let tss_descriptor1 = Descriptor::tss_segment(&tss1.task_state_segment);
        let code_selector = global_descriptor_table.append(code_selector1);
        let tss_selector = global_descriptor_table.append(tss_descriptor1);
        let data_selector = global_descriptor_table.append(Descriptor::kernel_data_segment());
        return GDT{global_descriptor_table,tss_selector,code_selector,data_selector};
    }
}

static GDT_INSTANCE: Once<GDT> = Once::new();

