use falcon_key_file::Callback;
use log::*;

pub fn invoke(callback: Callback) {
    debug!("Would now invoke {:?} as a keyboard", callback);
}

#[cfg(test)]
mod keyboard_emulator {
    #[test]
    fn it_works() {}
}
