use falcon_key_file::Callback;
use falcon_key_file::Key;
use falcon_key_file::Modifier;
use log::*;
use std::thread;
use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYBDINPUT, KEYEVENTF_EXTENDEDKEY,
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, SendInput, VIRTUAL_KEY,
};

const DW_EXTRA_INFO: usize = 123234;

pub fn invoke(callback: Callback) {
    debug!("Invoking {:?} as a keyboard.", callback);

    if callback.combo_key_code != 0 {
        invoke_keycode_with_modifiers(
            callback.combo_modifiers,
            callback.combo_key_code,
            callback.readable_combo_key_code,
        );
        thread::sleep(Duration::from_millis(30));
    }
    invoke_keycode_with_modifiers(
        callback.modifiers,
        callback.key_code,
        callback.readable_key_code,
    );
}

fn invoke_keycode_with_modifiers(modifiers: Vec<Modifier>, scan: u16, key: Key) {
    let modifiers: Vec<_> = modifiers.iter().map(to_key).collect();

    for modifier in modifiers.iter() {
        self::raw(*modifier, Direction::Press, false);
    }
    thread::sleep(Duration::from_millis(10));

    self::raw(scan, Direction::Click, self::is_extended_key(key));

    for modifier in modifiers.iter().rev() {
        self::raw(*modifier, Direction::Release, false);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Press,
    Release,
    Click,
}

type ScanCode = u16;

fn raw(scan: u16, direction: Direction, extended: bool) {
    debug!("\x1b[35mraw(scan: {scan:?}, direction: {direction:?})\x1b[0m");
    let mut input = vec![];

    let vk: VIRTUAL_KEY = VIRTUAL_KEY(0);

    let mut keyflags = KEYEVENTF_SCANCODE;

    if extended {
        keyflags |= KEYEVENTF_EXTENDEDKEY;
    }

    if direction == Direction::Click || direction == Direction::Press {
        input.push(keybd_event(keyflags, vk, scan, DW_EXTRA_INFO));
    }
    if direction == Direction::Click || direction == Direction::Release {
        input.push(keybd_event(
            keyflags | KEYEVENTF_KEYUP,
            vk,
            scan,
            DW_EXTRA_INFO,
        ));
    }

    send_input(&input).expect("Failed to send input.");
}

fn is_extended_key(key: Key) -> bool {
    // THE special snowflake keycode
    key == Key::NumpadEnter
}

fn keybd_event(
    flags: KEYBD_EVENT_FLAGS,
    vk: VIRTUAL_KEY,
    scan: ScanCode,
    dw_extra_info: usize,
) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: scan,
                dwFlags: flags,
                time: 0, /* Always set it to 0 (see https://web.archive.org/web/20231004113147/https://devblogs.microsoft.com/oldnewthing/20121101-00/?p=6193) */
                dwExtraInfo: dw_extra_info,
            },
        },
    }
}

fn send_input(input: &[INPUT]) -> InputResult<()> {
    if input.is_empty() {
        return Ok(());
    }
    let Ok(input_size): Result<i32, _> = size_of::<INPUT>().try_into() else {
        return Err(InputError::InvalidInput(
            "the size of the INPUT was so large, the size exceeded i32::MAX",
        ));
    };
    let Ok(input_len): Result<u32, _> = input.len().try_into() else {
        return Err(InputError::InvalidInput(
            "the number of INPUT was so large, the length of the Vec exceeded i32::MAX",
        ));
    };
    if unsafe { SendInput(input, input_size) } == input_len {
        Ok(())
    } else {
        let last_err = std::io::Error::last_os_error();
        error!("{last_err}");
        Err(InputError::Simulate(
            "not all input events were sent. they may have been blocked by UIPI",
        ))
    }
}

pub type InputResult<T> = Result<T, InputError>;

/// Error when simulating input
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputError {
    /// There was an error with the protocol
    Simulate(&'static str),
    /// The input you want to simulate is invalid
    /// This happens for example if you want to enter text that contains NULL
    /// bytes (`\0`)
    InvalidInput(&'static str),
}

fn to_key(m: &Modifier) -> u16 {
    match m {
        // we're going straight to the raw scancode for these, as per here:
        // https://gist.github.com/arithex/3e953d1eb096afe58ce05ba6846493e4
        Modifier::LSHIFT => 0x2A,
        Modifier::LCONTROL => 0x1D,
        Modifier::LALT => 0x38,
    }
}
