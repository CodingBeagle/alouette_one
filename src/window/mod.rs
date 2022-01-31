use windows::{
    Win32::{
        UI::WindowsAndMessaging::{GetCursorPos, SetCursorPos, GetForegroundWindow, GetWindowRect},
        Foundation::{HWND, RECT, POINT}
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
    S,
    Escape
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
        0x1B => {
            Key::Escape
        },
        _ => Key::Unknown
    }
}

#[derive(Default)]
pub struct Window {
    pub lock_cursor_center: bool,

    pub current_keyboard_state: HashSet<Key>,
    pub previous_keyboard_state: HashSet<Key>,
    pub mouse_move_x: i32,
    pub mouse_move_y: i32,
    // TODO: hwnd should perhaps be hidden behind a constructor. Public for now.
    pub hwnd: HWND,
    
    previous_mouse_position_x: i32,
    previous_mouse_position_y: i32,

    mouse_center_position_x: i32,
    mouse_center_position_y: i32
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

    pub fn center_cursor(&mut self) {
        if self.lock_cursor_center {
            unsafe {
                // I only want to center the cursor when the game window is the active / foreground window.
                let current_foreground_window = GetForegroundWindow();
                if current_foreground_window != 0 {
                    if self.hwnd == current_foreground_window {
                        // When setting the cursor position, it is done in "screen coordinates". Not relative to the window, but relative to the screen as a whole.
                        // Therefore, I first need to get the screen coordinates of the upper-left corner of the window.
                        let mut window_rectangle: RECT = RECT::default();
                        let success = GetWindowRect(self.hwnd, &mut window_rectangle as *mut RECT);
                        if success.0 == 0 {
                            panic!("Failed to retrieve window rectangle.")
                        }

                        self.mouse_center_position_x = window_rectangle.left + (800 / 2);
                        self.mouse_center_position_y = window_rectangle.top + (600 / 2);
    
                        // TODO: Move window dimensions to window struct, instead of values everywhere in the code!
                        SetCursorPos(self.mouse_center_position_x, self.mouse_center_position_y);
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        unsafe {
            let mut mouse_position: POINT = POINT::default();
            let success = GetCursorPos(&mut mouse_position as *mut POINT);
            if !success.as_bool() {
                panic!("Failed to retrieve cursor position!");
            }

            self.mouse_move_x = mouse_position.x - self.previous_mouse_position_x;
            self.mouse_move_y = mouse_position.y - self.previous_mouse_position_y;
    
            self.center_cursor();

            self.previous_mouse_position_x = if self.lock_cursor_center { self.mouse_center_position_x } else { mouse_position.x };
            self.previous_mouse_position_y = if self.lock_cursor_center { self.mouse_center_position_y } else { mouse_position.y };
            
            self.previous_keyboard_state = self.current_keyboard_state.clone();
        }
    }
}