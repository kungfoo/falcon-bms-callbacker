use log::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn parse(name: String, key_file: &File) -> Result<FalconKeyfile, KeyFileError> {
    let reader = BufReader::new(key_file);
    let mut lines = reader.lines();

    if lines.next().is_none() {
        return Err(KeyFileError::Empty);
    }

    let mut ln = 0;
    let mut keycodes_by_callback: HashMap<String, Callback> = HashMap::new();

    for line in lines {
        ln += 1;
        let line = line.map_err(KeyFileError::ReadError)?;
        if line.is_empty() {
            continue;
        }

        if !(line.starts_with("#") || line.starts_with("SimDoNothing")) {
            let stuff: Vec<&str> = line.split_whitespace().collect();
            trace!("Parsing line {}, tokens: {:?}", ln, stuff);

            // an actual key callback
            let callback_name = stuff[0];
            if callback_name != "SimDoNothing" {
                let error = format!(
                    "Expected key identifier to be u32 on line {} but was {}",
                    ln, stuff[2]
                );
                let is_key: i64 = stuff[2].parse().expect(&error);
                if is_key == 0 {
                    let callback = Callback {
                        name: String::from(callback_name),
                        key_code: convert_number(stuff[3]),
                        readable_key_code: parse_key_code(convert_number(stuff[3])),
                        modifiers: parse_modifiers(convert_number(stuff[4])),
                        combo_key_code: convert_number(stuff[5]),
                        readable_combo_key_code: parse_key_code(convert_number(stuff[5])),
                        combo_modifiers: parse_modifiers(convert_number(stuff[6])),
                    };
                    trace!("Parsed callback: {:?}", callback);
                    keycodes_by_callback.insert(String::from(callback_name), callback);
                }
            }
        }
    }

    debug!(
        "Parsed key file with {} callbacks.",
        keycodes_by_callback.keys().count()
    );

    Ok(FalconKeyfile::new(name, keycodes_by_callback))
}

#[derive(Debug, Clone)]
pub struct FalconKeyfile {
    name: String,
    callbacks: HashMap<String, Callback>,
}

impl FalconKeyfile {
    pub fn new(name: String, keycodes_by_callback: HashMap<String, Callback>) -> FalconKeyfile {
        FalconKeyfile {
            name,
            callbacks: keycodes_by_callback,
        }
    }

    pub fn callback(&self, callback_name: &str) -> Option<Callback> {
        match self.callbacks.get(callback_name) {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }

    pub fn describe(&self) -> String {
        format!(
            "{} with {} callbacks.",
            self.name,
            self.callbacks.keys().len()
        )
    }
}

#[derive(Debug, Clone)]
pub struct Callback {
    pub name: String,
    pub key_code: u16,
    pub readable_key_code: String,
    pub modifiers: Vec<Modifier>,
    pub combo_key_code: u16,
    pub readable_combo_key_code: String,
    pub combo_modifiers: Vec<Modifier>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Modifier {
    LSHIFT,
    LCONTROL,
    LALT,
}

#[derive(Debug)]
pub enum KeyFileError<'a> {
    Empty,
    ReadError(std::io::Error),
    ParseError(&'a str),
}

fn convert_number(number: &str) -> u16 {
    let number = number.to_lowercase();
    if number.starts_with("0x") {
        let without_prefix = number.trim_start_matches("0x");
        return u32::from_str_radix(without_prefix, 16).expect("Expected hex key code to be an u16")
            as u16;
    }
    number.parse().expect(&format!(
        "Expected key code number to be u32 but was '{}'",
        number
    ))
}

fn parse_modifiers(number: u16) -> Vec<Modifier> {
    let mut result = vec![];
    if number & 1 == 1 {
        result.push(Modifier::LSHIFT);
    }
    if number & 2 == 2 {
        result.push(Modifier::LCONTROL);
    }
    if number & 4 == 4 {
        result.push(Modifier::LALT);
    }
    result
}

fn parse_key_code(number: u16) -> String {
    let result = match number {
        1 => "ESCAPE",
        2 => "1",
        3 => "2",
        4 => "3",
        5 => "4",
        6 => "5",
        7 => "6",
        8 => "7",
        9 => "8",
        10 => "9",
        11 => "0",
        12 => "-",
        13 => "=",
        14 => "BACKSPACE",
        15 => "TAB",
        16 => "q",
        17 => "w",
        18 => "e",
        19 => "r",
        20 => "t",
        21 => "y",
        22 => "u",
        23 => "i",
        24 => "o",
        25 => "p",
        26 => "[",
        27 => "]",
        28 => "RETURN",
        29 => "`",
        30 => "a",
        31 => "s",
        32 => "d",
        33 => "f",
        34 => "g",
        35 => "h",
        36 => "j",
        37 => "k",
        38 => "l",
        39 => ";",
        40 => "'",
        41 => "`",
        43 => "\\",
        44 => "z",
        45 => "x",
        46 => "c",
        47 => "v",
        48 => "b",
        49 => "n",
        50 => "m",
        51 => ",",
        52 => ".",
        53 => "/",
        55 => "MULTIPLY",
        57 => "SPACE",
        58 => "CAPSLOCK",
        59 => "F1",
        60 => "F2",
        61 => "F3",
        62 => "F4",
        63 => "F5",
        64 => "F6",
        65 => "F7",
        66 => "F8",
        67 => "F9",
        68 => "F10",
        69 => "NUMLOCK",
        70 => "SCROLLOCK",
        71 => "NUMPAD7",
        72 => "NUMPAD8",
        73 => "NUMPAD9",
        74 => "SUBTRACT",
        75 => "NUMPAD4",
        76 => "NUMPAD5",
        77 => "NUMPAD6",
        78 => "ADD",
        79 => "NUMPAD1",
        80 => "NUMPAD2",
        81 => "NUMPAD3",
        82 => "NUMPAD0",
        83 => "DECIMAL",
        87 => "F11",
        88 => "F12",
        100 => "F13",
        101 => "F14",
        102 => "F15",
        156 => "NUMPADENTER",
        157 => "RCONTROL",
        181 => "DIVIDE",
        183 => "SYSRQ",
        199 => "HOME",
        200 => "UP",
        201 => "PAGEUP",
        203 => "LEFT",
        205 => "RIGHT",
        207 => "END",
        208 => "DOWN",
        209 => "PAGEDOWN",
        210 => "INSERT",
        211 => "DELETE",
        219 => "LWIN",
        220 => "RWIN",
        221 => "APPS",
        0 => "",
        0xFFFF => "",
        e => todo!("Unmatched key code {}", e),
    };
    String::from(result)
}

#[cfg(test)]
mod falcon_key_file {
    use super::*;
    use std::path::Path;

    #[test]
    fn ensure_binary_ops() {
        assert_eq!(6 & 4, 4);
        assert_eq!(6 & 2, 2);

        assert_eq!(3 & 2, 2);
        assert_eq!(3 & 1, 1);
    }

    #[test]
    fn parse_basic_key_file() {
        let path = Path::new("src/lib/falcon-key-file/test-data/basic.key");
        let file = File::open(&path).unwrap();
        let result = parse(String::from("basic.key"), &file);
        assert!(result.is_ok());

        let result = result.unwrap();

        let callback = result.callback("AFElevatorTrimUp");
        assert!(callback.is_some());
        let callback = callback.unwrap();
        println!("{:?}", callback);
        assert_eq!(callback.readable_key_code, "UP");
        assert_eq!(callback.modifiers, vec![Modifier::LCONTROL]);

        let callback = result.callback("OTWBalanceIVCvsAIUp").unwrap();
        assert_eq!(callback.readable_key_code, "]");

        let callback = result.callback("OTWBalanceIVCvsAIDown").unwrap();
        assert_eq!(callback.readable_key_code, "[");

        // let's find one with multiple modifiers
        let callback = result.callback("AFElevatorUp").unwrap();
        assert_eq!(callback.readable_key_code, "UP");
        assert_eq!(
            callback.modifiers,
            vec![Modifier::LSHIFT, Modifier::LCONTROL]
        );

        // let's find a combo key
        let callback = result.callback("SimPilotToggle").unwrap();
        assert_eq!(callback.readable_key_code, "p");
        assert!(callback.modifiers.is_empty());

        assert_eq!(callback.readable_combo_key_code, "c");
        assert_eq!(callback.combo_modifiers, vec![Modifier::LALT]);

        // let's find another combo key
        let callback = result.callback("OTWToggleFrameRate").unwrap();
        assert_eq!(callback.readable_key_code, "f");
        assert!(callback.modifiers.is_empty());

        assert_eq!(callback.readable_combo_key_code, "c");
        assert_eq!(callback.combo_modifiers, vec![Modifier::LALT]);
    }
}
