use winapi::um::winuser::{GetAsyncKeyState, VK_MBUTTON, mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, VK_F8};
use std::thread::sleep;
use winapi::_core::time::Duration;

#[derive(Default)]
struct Clicker {
    enabled: bool
}

impl Clicker {
    pub fn new() -> Self {
      Default::default()
    }
}

fn main() {
    let mut clicker = Clicker::new();
    unsafe {
        loop {
            if (GetAsyncKeyState(VK_MBUTTON) & 1) == 1 {
                clicker.enabled.inverse();
            }
            if (GetAsyncKeyState(VK_F8) & 1) == 1 {
                break;
            }
            if clicker.enabled {
                mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                sleep(Duration::from_millis(1000 / 50));
                mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
            }
        }
    }
}

trait Inverse {
    fn inverse(&mut self);
}

impl Inverse for bool {
    fn inverse(&mut self) {
        *self = !*self;
    }
}