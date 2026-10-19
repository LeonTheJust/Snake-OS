use x86_64::instructions::hlt;
use pic8259;
use spin::Mutex;
use core::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use core::sync::atomic::Ordering::Relaxed;
use pic8259::ChainedPics;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use spin::Once;
use uart_16550::{Config, Uart16550};
use x86_64::addr::VirtAddrNotValid;
use x86_64::VirtAddr;
use crate::serial;
use x86_64::instructions::port::PortReadOnly;
use crate::serial::u64_to_str;

pub struct InterruptHandler{
    interrupt_table : InterruptDescriptorTable,
}

impl InterruptHandler{
    pub fn new() -> Self{
        let mut interrupt_table = InterruptDescriptorTable::new();
        interrupt_table.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe{interrupt_table.double_fault.set_handler_fn(double_fault_handler).set_stack_index(0);}
        interrupt_table.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        interrupt_table.page_fault.set_handler_fn(page_fault_handler);
        interrupt_table[32].set_handler_fn(timer_handler);
        interrupt_table[33].set_handler_fn(keyboard_handler);
        return InterruptHandler{interrupt_table};
    }

    pub fn load(&'static self){
        InterruptDescriptorTable::load(&self.interrupt_table);
    }

}

static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(32, 40) });
static INTERRUPT_HANDLER: Once<InterruptHandler> = Once::new();
pub fn init() {
    let handler: &'static InterruptHandler = INTERRUPT_HANDLER.call_once(InterruptHandler::new);
    unsafe{PICS.lock().initialize();
        PICS.lock().write_masks(0b1111_1100, 0b1111_1111);
    }
    handler.load();
    x86_64::instructions::interrupts::enable();
}

pub(crate) static flag: AtomicBool = AtomicBool::new(false);
pub(crate) static flag2: AtomicBool = AtomicBool::new(false);

pub (crate) static tick_counter : AtomicU64 = AtomicU64::new(0);
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
   AtomicBool::store(&flag, true,Ordering::SeqCst);
    //emergency_message(stack_frame.stack_segment.index() as u64);
    //emergency_message();
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    let mut buffer = [0u8;20];
    //serial::emergency_message(u64_to_str(stack_frame.code_segment.index() as u64, &mut buffer));
    serial::emergency_message(u64_to_str(error_code, &mut buffer));
    AtomicBool::store(&flag2, true,Ordering::SeqCst);
    loop {
        x86_64::instructions::hlt();
    }
}

extern "x86-interrupt" fn general_protection_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    //emergency_message(error_code);
    let mut buffer = [0u8;20];
    serial::emergency_message(u64_to_str(stack_frame.code_segment.index() as u64, &mut buffer));
    loop {
        x86_64::instructions::hlt();
    }
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    let result = x86_64::registers::control::Cr2::read();
    let mut buffer = [0u8;20];
    serial::emergency_message("Error Code: ");
    serial::emergency_message(u64_to_str(error_code.bits(), &mut buffer));
    serial::emergency_message("\nInstruction pointer: ");
    serial::emergency_message(u64_to_str(stack_frame.instruction_pointer.as_u64(), &mut buffer));
    serial::emergency_message("\nFaulting address: ");
    match result{
        Ok(result) => {
            let addr = result.as_u64();
            serial::emergency_message(serial::u64_to_str(addr,&mut buffer));
        }
        Err(result) => {
            let error = result.0;
            serial::emergency_message(serial::u64_to_str(error,&mut buffer));
        }
    }
    //emergency_message(result.)
    loop {
        x86_64::instructions::hlt();
    }
}

extern "x86-interrupt" fn timer_handler(stack_frame: InterruptStackFrame) {
    tick_counter.store(tick_counter.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
    unsafe{PICS.lock().notify_end_of_interrupt(32);}
}

pub (crate) static KEYBOARD: AtomicU8 = AtomicU8::new(0);

extern "x86-interrupt" fn keyboard_handler(stack_frame: InterruptStackFrame) {
    let mut port = PortReadOnly::<u8>::new(0x60);
    let scancode = unsafe { port.read() };
    KEYBOARD.store(scancode, Ordering::Relaxed);
    unsafe{PICS.lock().notify_end_of_interrupt(33);}
}