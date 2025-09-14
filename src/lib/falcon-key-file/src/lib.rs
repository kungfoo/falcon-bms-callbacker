use enigo::Key;
use levenshtein::levenshtein;
use log::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn parse(name: String, key_file: &'_ File) -> Result<FalconKeyfile, KeyFileError<'_>> {
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

    pub fn propose_callback_names(&self, query: String, count: usize) -> Vec<String> {
        let mut names: Vec<_> = self.callbacks.keys().cloned().collect();
        names.sort_by(|a, b| levenshtein(&query, a).cmp(&levenshtein(&query, b)));

        return names.iter().take(count).map(|s| String::from(s)).collect();
    }
}

#[derive(Debug, Clone)]
pub struct Callback {
    pub name: String,
    pub key_code: u16,
    pub readable_key_code: Key,
    pub modifiers: Vec<Modifier>,
    pub combo_key_code: u16,
    pub readable_combo_key_code: Key,
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

fn parse_key_code(number: u16) -> Key {
    return match number {
        // these are maric unicorns in keyfiles
        0 => Key::Other(0),
        0xFFFF => Key::Other(0),
        // normal ones start here:
        1 => Key::Escape,
        2 => Key::Num1,
        3 => Key::Num2,
        4 => Key::Num3,
        5 => Key::Num4,
        6 => Key::Num5,
        7 => Key::Num6,
        8 => Key::Num7,
        9 => Key::Num8,
        10 => Key::Num9,
        11 => Key::Num0,
        12 => Key::Unicode('-'),
        13 => Key::Unicode('='),
        14 => Key::Backspace,
        15 => Key::Tab,
        16 => Key::Q,
        17 => Key::W,
        18 => Key::E,
        19 => Key::R,
        20 => Key::T,
        21 => Key::Y,
        22 => Key::U,
        23 => Key::I,
        24 => Key::O,
        25 => Key::P,
        26 => Key::Unicode('['),
        27 => Key::Unicode(']'),
        28 => Key::Return,
        29 => Key::LControl,
        30 => Key::A,
        31 => Key::S,
        32 => Key::D,
        33 => Key::F,
        34 => Key::G,
        35 => Key::H,
        36 => Key::J,
        37 => Key::L,
        38 => Key::L,
        39 => Key::Unicode(';'),
        40 => Key::Unicode('\''),
        41 => Key::Unicode('`'),
        42 => Key::LShift,
        43 => Key::Unicode('\\'),
        44 => Key::Z,
        45 => Key::X,
        46 => Key::C,
        47 => Key::V,
        48 => Key::B,
        49 => Key::N,
        50 => Key::M,
        51 => Key::Unicode(','),
        52 => Key::Unicode('.'),
        53 => Key::Unicode('/'),
        55 => Key::Multiply,
        57 => Key::Space,
        58 => Key::CapsLock,
        59 => Key::F1,
        60 => Key::F2,
        61 => Key::F3,
        62 => Key::F4,
        63 => Key::F5,
        64 => Key::F6,
        65 => Key::F7,
        66 => Key::F8,
        67 => Key::F9,
        68 => Key::F10,
        69 => Key::Numlock,
        70 => Key::Scroll,
        71 => Key::Numpad7,
        72 => Key::Numpad8,
        73 => Key::Numpad9,
        74 => Key::Subtract,
        75 => Key::Numpad4,
        76 => Key::Numpad5,
        77 => Key::Numpad6,
        78 => Key::Add,
        79 => Key::Numpad1,
        80 => Key::Numpad2,
        81 => Key::Numpad3,
        82 => Key::Numpad0,
        83 => Key::Decimal,
        87 => Key::F11,
        88 => Key::F12,
        100 => Key::F13,
        101 => Key::F14,
        102 => Key::F15,
        156 => Key::Separator,
        157 => Key::RControl,
        181 => Key::Divide,
        183 => Key::PrintScr,
        199 => Key::Home,
        200 => Key::UpArrow,
        201 => Key::PageUp,
        203 => Key::LeftArrow,
        205 => Key::RightArrow,
        207 => Key::End,
        208 => Key::DownArrow,
        209 => Key::PageDown,
        210 => Key::Insert,
        211 => Key::Delete,
        219 => Key::LWin,
        220 => Key::RWin,
        221 => Key::Apps,
        e => todo!("Unmatched key code {}", e),
    };
}

#[cfg(test)]
mod falcon_key_file {
    use super::*;
    use env_logger;
    use env_logger::Env;
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
        assert_eq!(callback.readable_key_code, Key::UpArrow);
        assert_eq!(callback.modifiers, vec![Modifier::LCONTROL]);

        let callback = result.callback("AFBrakesToggle").unwrap();
        assert_eq!(callback.readable_key_code, Key::B);
        assert_eq!(callback.modifiers, vec![]);

        let callback = result.callback("OTWBalanceIVCvsAIUp").unwrap();
        assert_eq!(callback.readable_key_code, Key::Unicode(']'));

        let callback = result.callback("OTWBalanceIVCvsAIDown").unwrap();
        assert_eq!(callback.readable_key_code, Key::Unicode('['));

        // let's find one with multiple modifiers
        let callback = result.callback("AFElevatorUp").unwrap();
        assert_eq!(callback.readable_key_code, Key::UpArrow);
        assert_eq!(
            callback.modifiers,
            vec![Modifier::LSHIFT, Modifier::LCONTROL]
        );

        // let's find a combo key
        let callback = result.callback("SimPilotToggle").unwrap();
        assert_eq!(callback.readable_key_code, Key::P);
        assert!(callback.modifiers.is_empty());

        assert_eq!(callback.readable_combo_key_code, Key::C);
        assert_eq!(callback.combo_modifiers, vec![Modifier::LALT]);

        // let's find another combo key
        let callback = result.callback("OTWToggleFrameRate").unwrap();
        assert_eq!(callback.readable_key_code, Key::F);
        assert!(callback.modifiers.is_empty());

        assert_eq!(callback.readable_combo_key_code, Key::C);
        assert_eq!(callback.combo_modifiers, vec![Modifier::LALT]);
    }

    #[test]
    fn parse_t16000m_key_file() {
        let env = Env::default().filter_or("LOG_LEVEL", "debug");
        env_logger::init_from_env(env);

        let path = Path::new("src/lib/falcon-key-file/test-data/T16000M-FCS-Full.key");
        let file = File::open(&path).unwrap();
        let result = parse(String::from("T16000M-FCS-Full.key"), &file);
        assert!(result.is_ok());

        let result = result.unwrap();

        // find one callback with SLASH
        let callback = result.callback("SimMissileStep");
        assert!(callback.is_some());
        let callback = callback.unwrap();
        println!("{:?}", callback);
        assert_eq!(callback.readable_key_code, Key::Unicode('/'));
        assert_eq!(callback.modifiers, vec![Modifier::LSHIFT]);

        // let's find the problematic new ones
        let callback = result.callback("SimMIDSLVTInc");
        assert!(callback.is_some());
        let callback = callback.unwrap();
        assert_eq!(callback.readable_key_code, Key::Unicode('/'));
        assert_eq!(callback.modifiers, vec![Modifier::LSHIFT, Modifier::LALT]);
    }
}
