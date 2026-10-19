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

pub (crate) fn emergency_message(s : &str ) {
    let mut uart = unsafe { Uart16550::new_port(0x3f8).unwrap() };
    uart.send_bytes_exact(s.as_bytes());
}

pub(crate) fn u64_to_str<'a>(mut value: u64, buffer: &'a mut [u8; 20]) -> &'a str {
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





