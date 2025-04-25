#[derive(Debug)]
pub enum Io {
    Error(String),
    Keyring(io_keyring::Io),
    Command(io_process::Io),
}
