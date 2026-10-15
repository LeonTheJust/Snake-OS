use x86_64::instructions::hlt;
use core::sync::atomic::{AtomicBool, Ordering};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use spin::Once;
use uart_16550::{Config, Uart16550};
use crate::serial;

pub struct InterruptHandler{
    interrupt_table : InterruptDescriptorTable,
}

impl InterruptHandler{
    pub fn new() -> Self{
        let mut interrupt_table = InterruptDescriptorTable::new();
        interrupt_table.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe{interrupt_table.double_fault.set_handler_fn(double_fault_handler).set_stack_index(0);}
        interrupt_table.general_protection_fault.set_handler_fn(general_protection_fault_handler);
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
pub(crate) static flag2: AtomicBool = AtomicBool::new(false);

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
   AtomicBool::store(&flag, true,Ordering::SeqCst);
    //emergency_message(stack_frame.stack_segment.index() as u64);
    //emergency_message();
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    //emergency_message();
    AtomicBool::store(&flag2, true,Ordering::SeqCst);
    loop {
        x86_64::instructions::hlt();
    }
}

extern "x86-interrupt" fn general_protection_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    //emergency_message(error_code);
    //emergency_message(stack_frame.code_segment.index() as u64);
    loop {
        x86_64::instructions::hlt();
    }
}

fn u64_to_str<'a>(mut value: u64, buffer: &'a mut [u8; 20]) -> &'a str {
    let mut start = buffer.len();

    loop {
        start -= 1;
        buffer[start] = b'0' + (value % 10) as u8;
        value /= 10;

        if value == 0 {
            break;
        }
    }

    core::str::from_utf8(&buffer[start..]).unwrap()
}
fn  emergency_message(error : u64){
    let mut uart = unsafe { Uart16550::new_port(0x3f8).unwrap() };
    uart.init(Config::default()).expect("should init device successfully");
    uart.send_bytes_exact(b"Serial initialised");
    uart.send_bytes_exact(("EMERGENCY\n").as_bytes());
    let mut buffer = [0u8; 20];
    let error_text = u64_to_str(error, &mut buffer);
    uart.send_bytes_exact(error_text.as_bytes());

}