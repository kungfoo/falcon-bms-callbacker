use falcon_key_file::FalconKeyfile;

#[derive(Debug)]
pub enum Message {
    CallbackReceived { callback: String },

    KeyfileRead { key_file: FalconKeyfile },
}
