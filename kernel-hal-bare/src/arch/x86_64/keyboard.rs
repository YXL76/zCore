use lazy_static::lazy_static;
use linux_object::fs::{InputEvent, INPUT_EVENT};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;

/// Receive character from keyboard
/// Should be called on every interrupt
pub fn receive() -> Option<DecodedKey> {
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut data_port = Port::<u8>::new(0x60);
    let mut status_port = Port::<u8>::new(0x64);

    // Output buffer status = 1
    if unsafe { status_port.read() } & 1 != 0 {
        let scancode = unsafe { data_port.read() };
        INPUT_EVENT.lock().push_back(match scancode {
            0x80..=0xFF => InputEvent::new(1, (scancode as u16) - 0x80, 0),
            _ => InputEvent::new(1, scancode as u16, 1),
        });
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            return keyboard.process_keyevent(key_event);
        }
    }
    None
}
