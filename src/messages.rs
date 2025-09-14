use falcon_key_file::FalconKeyfile;

#[derive(Debug)]
pub enum Message {
    /// Sent when we received a callback
    CallbackReceived {
        callback: String,
    },

    // Sent whenever a new kezfile has been read
    KeyfileRead {
        key_file: FalconKeyfile,
    },
}
