# Keyboard Input

Good learning source: <https://docs.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input>

Keyboards produce scan-codes when keys are pressed and released.

Scan codes are device *dependent* values indicating what button was pressed on the keyboard.

A keyboard driver translates the scan code to a **virtual-key code**, which is a device *independent* value defined by Windows that identifies the purpose of a key (if it's the up-arrow button, A button, etc...).

The keyboard driver will take the scan code, virtual key code, and other information about the keystroke, and pass it to the system message queue of Windows, which will pass it to a specific application's thread message queue, which can then handle it in its *Window Procedure*.