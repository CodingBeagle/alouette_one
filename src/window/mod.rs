use windows::{
    Win32::{
        UI::WindowsAndMessaging::{SetCursorPos, GetForegroundWindow},
        Foundation::{HWND}
    }
};

use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Key {
    Unknown,
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    A,
    D,
    W,
    S
}

pub fn map_to_key(virtual_key_code: i32) -> Key {
    // https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    match virtual_key_code {
        0x41 => {
            Key::A
        },
        0x44 => {
            Key::D
        },
        0x53 => {
            Key::S
        },
        0x57 => {
            Key::W
        },
        _ => Key::Unknown
    }
}

#[derive(Default)]
pub struct Window {
    pub current_keyboard_state: HashSet<Key>,
    pub previous_keyboard_state: HashSet<Key>,

    // TODO: hwnd should perhaps be hidden behind a constructor. Public for now.
    pub hwnd: HWND
}

impl Window {
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.current_keyboard_state.contains(&key)
    }

    pub fn was_key_pressed(&self, key: Key) -> bool {
        self.current_keyboard_state.contains(&key) && !self.previous_keyboard_state.contains(&key)
    }
    
    pub fn was_key_released(&self, key: Key) -> bool {
        !self.current_keyboard_state.contains(&key) && self.previous_keyboard_state.contains(&key)
    }

    pub fn center_cursor(&self) {
        unsafe {
            // I only want to center the cursor when the game window is the active / foreground window.
            let current_foreground_window = GetForegroundWindow();
            if current_foreground_window != 0 {
                if self.hwnd == current_foreground_window {
                    // TODO: Move window dimensions to window struct, instead of values everywhere in the code!
                    SetCursorPos(800 / 2, 600 / 2);
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.previous_keyboard_state = self.current_keyboard_state.clone();
    }
}