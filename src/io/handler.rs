use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub trait IoDevice {
    fn read(&self) -> u8;
    fn write(&mut self, value: u8);
}

pub struct IoHandler {
    devices: HashMap<u8, Rc<RefCell<dyn IoDevice>>>,
}

impl IoHandler {
    pub fn new() -> Self {
        IoHandler { devices: HashMap::new() } 
    }

    pub fn register_device<T: IoDevice + 'static> (&mut self, port: u8, device: T) {
        self.devices.insert(port, Rc::new(RefCell::new(device)));
    }

    pub fn read(&self, port: u8) -> u8 {
        if let Some(device) = self.devices.get(&port) {
            device.borrow().read()
        } else { 0x00 }
    }

    pub fn write(&self, port: u8, value: u8) {
        if let Some(device) = self.devices.get(&port) {
            device.borrow_mut().write(value);
        }
    }
}