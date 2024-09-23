use enigo::Key::LControl;
use enigo::Key::LMenu;
use enigo::Key::LShift;
use falcon_key_file::Callback;
use falcon_key_file::Modifier;
use log::*;
use std::thread;
use tokio::time::Duration;

use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};

pub fn invoke(callback: Callback) {
    debug!("Invoking {:?} as a keyboard.", callback);

    let settings = Settings {
        windows_dw_extra_info: Some(13987),
        ..Default::default()
    };
    let mut enigo = Enigo::new(&settings).unwrap();

    if callback.combo_key_code != 0 {
        invoke_keycode_with_modifiers(
            &mut enigo,
            callback.combo_modifiers,
            callback.combo_key_code,
        );
        thread::sleep(Duration::from_millis(30));
    }
    invoke_keycode_with_modifiers(&mut enigo, callback.modifiers, callback.key_code);
}

fn invoke_keycode_with_modifiers(enigo: &mut Enigo, modifiers: Vec<Modifier>, key_code: u16) {
    let modifiers: Vec<_> = modifiers.iter().map(to_key).collect();

    for modifier in modifiers.iter() {
        enigo.key(*modifier, Press).ok();
    }
    thread::sleep(Duration::from_millis(50));
    enigo.raw(key_code, Click).ok();

    for modifier in modifiers.iter().rev() {
        enigo.key(*modifier, Release).ok();
    }
}

fn to_key(m: &Modifier) -> Key {
    match m {
        Modifier::LSHIFT => LShift,
        Modifier::LCONTROL => LControl,
        Modifier::LALT => LMenu,
    }
}
