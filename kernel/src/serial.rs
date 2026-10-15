use x86_64::instructions::port::Port;
use uart_16550::{Config, Uart16550};
use uart_16550::backend::PioBackend;

pub struct Serial{
    uart: Uart16550<PioBackend>,

}
impl Serial {pub(crate) fn new() -> Self{
        let mut uart = unsafe { Uart16550::new_port(0x3f8).unwrap() };
        uart.init(Config::default()).expect("should init device successfully");
        uart.send_bytes_exact(b"Serial initialised");
        return Self{uart};
    }
    pub(crate) fn write_message(&mut self, message: &str ) {
        self.uart.send_bytes_exact(message.as_bytes());
    }

}







