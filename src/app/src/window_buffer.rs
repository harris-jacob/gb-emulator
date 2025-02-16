use std::cell::UnsafeCell;

use emulator_core::Renderer;

use crate::{HEIGHT, WIDTH};

pub struct WindowBuffer {
    buffer: UnsafeCell<[u32; WIDTH * HEIGHT]>,
}

unsafe impl Sync for WindowBuffer {}
unsafe impl Send for WindowBuffer {}

impl WindowBuffer {
    pub fn new() -> Self {
        Self {
            buffer: UnsafeCell::new([0; WIDTH * HEIGHT]),
        }
    }

    pub fn buffer(&self) -> &[u32; WIDTH * HEIGHT] {
        unsafe { &*self.buffer.get() }
    }
}

impl Renderer for WindowBuffer {
    fn render(&self, buffer: [u32; WIDTH * HEIGHT]) {
        unsafe {
            let value = self.buffer.get();
            *value = buffer;
        }
    }
}
