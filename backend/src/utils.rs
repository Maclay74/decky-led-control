#![allow(dead_code)]

mod utils {
    use std::arch::asm;

    pub fn outb(port: u16, data: u8) {
        unsafe { 
            asm!("out dx, al", in("dx") port, in("al") data, options(nostack)) 
        }
    }
    
    pub fn inb(port: u16) -> u8 {
        let value: u8;
        unsafe {
            asm!("in al, dx", out("al") value, in("dx") port)
        }
        value
    }
}