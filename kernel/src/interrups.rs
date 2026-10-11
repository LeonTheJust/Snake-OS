use core::sync::atomic::{AtomicBool, Ordering};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use spin::Once;

pub struct InterruptHandler{
    interrupt_table : InterruptDescriptorTable,
}

impl InterruptHandler{
    pub fn new() -> Self{
        let mut interrupt_table = InterruptDescriptorTable::new();
        interrupt_table.breakpoint.set_handler_fn(breakpoint_handler);
        return InterruptHandler{interrupt_table};
    }

    pub fn load(&'static self){
        InterruptDescriptorTable::load(&self.interrupt_table);
    }
}



static INTERRUPT_HANDLER: Once<InterruptHandler> = Once::new();
pub fn init() {
    let handler: &'static InterruptHandler = INTERRUPT_HANDLER.call_once(InterruptHandler::new);
    handler.load();
}

pub(crate) static flag: AtomicBool = AtomicBool::new(false);


extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame
) {
   AtomicBool::store(&flag, true,Ordering::SeqCst);
}