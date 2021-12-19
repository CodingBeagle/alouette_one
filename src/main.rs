use windows::{
    Win32::{System::*, UI::WindowsAndMessaging::*, Foundation::*}  
};

use std::{mem::{size_of}, os::windows::prelude::OsStrExt};
use std::ptr;
use std::ffi::*;
use core::iter::*;

fn main() {
    println!("Hello, world!");

    unsafe {
        // Retrieve module handle (a module being either a .exe file or DLL) for the .exe file.
        // When GetModuleHandleW is called with "None", it returns a handle for the .exe file.
        let h_instance = LibraryLoader::GetModuleHandleW(None);

        // Create a window class.
        // The window class defines the attributes of a window, like style, icon, cursor, menu, and
        // probably most importantly, the Window Procedure.
        // A Window Procedure MUST BE SET, otherwise "CreateWindow..." will fail.
        // You must register a window class, and then afterwards use that class to create a window.
        let mut window_class_name : Vec<u16> = OsStr::new("mainwindow").encode_wide().chain( once(0) ).collect();

        let mut window_class = WNDCLASSEXW::default();
        window_class.cbSize = size_of::<WNDCLASSEXW>() as u32;
        window_class.style = CS_HREDRAW | CS_VREDRAW;
        window_class.hInstance = h_instance;
        window_class.hCursor = LoadCursorW(h_instance, IDC_ARROW);
        window_class.lpszClassName = PWSTR(window_class_name.as_mut_ptr());
        window_class.lpfnWndProc = Some(wndproc);

        // If RegisterClassExW fails, 0 will be returned.
        if RegisterClassExW(&window_class) == 0 {
            panic!("Failed to register window class.");
        }

        // Create window
        // If successful, the function will return a handle  to the new window.
        // If the function fails, the return value will be zero (null).
        let mut window_title : Vec<u16> = OsStr::new("Alouette One").encode_wide().chain( once(0) ).collect();
        let main_window = CreateWindowExW(
            Default::default(),
            PWSTR(window_class_name.as_mut_ptr()),
            PWSTR(window_title.as_mut_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            0, 0, 800, 600,
            None,
            None,
            h_instance,
            ptr::null_mut()
        );

        if main_window == 0 {
            panic!("Failed to create window!");
        }

        let mut should_quit = false;

        let mut current_message = MSG::default();

        while !should_quit {
            // PROCESS INPUT
            // PeekMessage will retrieve messages associated with the main window and the thread.
            // I specify Null for hwnd because I want to not only retrieve messages associated with the window,
            // But also with the window's thread. This is so I can als ocatch messages like WM_QUIT.
            // By specifying PM_REMOVE, we remove the message from the queue for processing.
            if PeekMessageW(&mut current_message, None, 0, 0, PM_REMOVE) != false {
                if current_message.message == WM_QUIT {
                    should_quit = true;
                }

                // Translate virtual-key messages into character messages.
                // The character message is posted to the calling thread's message queue, to be read the next time the thread
                // Calls the GetMessage or PeekMessage function.
                // The message will be WM_CHAR, with wParam containing the character code of the key.
                TranslateMessage(&current_message);

                // Dispatch message to the window procedure.
                DispatchMessageW(&current_message);
            } else {
                // GAME LOOP
            }
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            // WM_CHAR is a message that is posed after calling TranslateMessage + DispatchMessage.
            // It contains the character encoding of whatever virtual-key was pressed
            // In the message's WPARAM.
            // WM_CHARs will not be generated for non-character keys (like arrow keys, delete, enter, etc...)
            WM_CHAR => {
                println!("Character key was pressed!");
                0
            },
            WM_DESTROY => {
                println!("Destroying window!");
                PostQuitMessage(0);
                0
            },
            _ => DefWindowProcW(window, message, wparam, lparam)
        }
    }
}