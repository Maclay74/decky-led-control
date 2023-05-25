#![allow(dead_code)]

use std::net::{TcpListener};
use std::io::{Read, Write};
use std::thread;
use std::env;
use std::fs::{OpenOptions};
use memmap::{MmapMut, MmapOptions};
use std::time::Duration;

mod utils;

trait LedController {
    fn init() -> Self where Self: Sized;
    fn probe() -> bool where Self: Sized;
    fn set_rgb(&mut self, _rgb: (u8, u8, u8)) {}
    fn supports_rgb(&self) -> bool { false }
}


struct AirLedController {
    map: MmapMut, 
}

impl AirLedController {
    const LEFT_JOYSTICK: u8 = 1;
    const RIGHT_JOYSTICK: u8 = 2;

    const RIGHT_LED: u8 = 1;
    const BOTTOM_LED: u8 = 2;
    const LEFT_LED: u8 = 3;
    const TOP_LED: u8 = 4;

    const AIR_EC_RAM_BASE: u64 = 0xFE800400;
    const AIR_EC_RAM_SIZE: usize = 0xFF;

    fn set_pixel(&mut self, js: u8, led: u8, color: (u8, u8, u8)) {
        self.ec_cmd(js, led * 3, color.0);
        self.ec_cmd(js, led * 3 + 1, color.1);
        self.ec_cmd(js, led * 3 + 2, color.2);
    }

    fn ec_cmd(&mut self, cmd: u8, p1: u8, p2: u8) {
        self.map[0x6d] = cmd;
        self.map[0xb1] = p1;
        self.map[0xb2] = p2;
        self.map[0xbf] = 0x10;
        thread::sleep(Duration::from_millis(10));
        self.map[0xbf] = 0xff;
        thread::sleep(Duration::from_millis(10));
    }
}

impl LedController for AirLedController {
    fn init() -> Self {
        let devmem = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("/dev/mem")
            .expect("Failed to open /dev/mem");

        unsafe {
            let map = MmapOptions::new()
                .offset(Self::AIR_EC_RAM_BASE)
                .len(Self::AIR_EC_RAM_SIZE)
                .map_mut(&devmem)
                .expect("Failed to mmap /dev/mem");

            let mut tmp = Self { map };
            tmp.ec_cmd(0x03, 0x02, 0x00);
            tmp
        }
    }

    fn probe() -> bool {
        true
    }

    fn supports_rgb(&self) -> bool {
        true
    }

    fn set_rgb(&mut self, color: (u8, u8, u8)) {
        self.set_pixel(Self::LEFT_JOYSTICK, Self::RIGHT_LED, color);
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let ip = &args[1];
    let port = &args[2];
    let address = format!("{}:{}", ip, port);

    // Set up a TCP listener to receive commands
    let listener = TcpListener::bind(&address).unwrap();
    thread::spawn(move || {
        loop {

            /*
            let color = (255 as u8, 0 as u8, 255 as u8);
            let mut controller = Box::new(AirLedController::init());
            controller.set_rgb(color);
            */
            
            // Sleep for a certain interval
            thread::sleep(std::time::Duration::from_secs(5));
        }
    });

    // Accept incoming connections and handle commands
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // Read the command sent from the client
                
                let mut buffer = vec![0; 1024];
                let bytes_read = stream.read(&mut buffer).unwrap();
                buffer.truncate(bytes_read);

                let command = String::from_utf8_lossy(&buffer[..]).trim().to_string();

                let parts: Vec<&str> = command.split(' ').collect();

                let r: u8 = parts[1].parse().unwrap();
                let g: u8 = parts[2].parse().unwrap();
                let b: u8 = parts[3].parse().unwrap();

                let mut controller = Box::new(AirLedController::init());
                controller.set_rgb((r, g, b));

                // You can access and modify the controller here
                // based on the received command

                // Send a response back to the client
                let response = "Command received totally";
                stream.write(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
