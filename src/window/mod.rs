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
    pub previous_keyboard_state: HashSet<Key>
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

    pub fn update(&mut self) {
        self.previous_keyboard_state = self.current_keyboard_state.clone();
    }
}