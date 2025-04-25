//! Module dedicated to the [`ReadFromCommand`] secret I/O-free coroutine.

use std::mem;

#[cfg(feature = "keyring")]
use io_keyring::coroutines::Read as ReadKeyringEntry;
#[cfg(feature = "command")]
use io_process::coroutines::SpawnThenWaitWithOutput;
use secrecy::SecretString;

use crate::{Io, Secret};

#[derive(Debug)]
enum SubRead {
    Raw(SecretString),
    #[cfg(feature = "keyring")]
    Keyring(ReadKeyringEntry),
    #[cfg(feature = "command")]
    Process(SpawnThenWaitWithOutput),
}

/// I/O-free coroutine for reading a secret from a shell command.
#[derive(Debug)]
pub struct Read {
    read: SubRead,
}

impl Read {
    pub fn new(secret: Secret) -> Self {
        let read = match secret {
            Secret::Raw(secret) => SubRead::Raw(secret),
            #[cfg(feature = "command")]
            Secret::Command(cmd) => SubRead::Process(SpawnThenWaitWithOutput::new(cmd)),
            #[cfg(feature = "keyring")]
            Secret::Keyring(entry) => SubRead::Keyring(ReadKeyringEntry::new(entry)),
        };

        Self { read }
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<SecretString, Io> {
        match &mut self.read {
            SubRead::Raw(secret) => Ok(mem::take(secret)),
            #[cfg(feature = "command")]
            SubRead::Process(spawn) => {
                let output = match input {
                    None => spawn.resume(None),
                    Some(Io::Command(io)) => spawn.resume(Some(io)),
                    Some(io) => {
                        let err = format!("expected command input, got {io:?}");
                        return Err(Io::Error(err));
                    }
                };

                match output {
                    Ok(output) => {
                        let first_line = match memchr::memchr(b'\n', &output.stdout) {
                            Some(n) => &output.stdout[..n],
                            None => &output.stdout,
                        };
                        let secret = String::from_utf8_lossy(first_line).to_string();
                        Ok(SecretString::from(secret))
                    }
                    Err(io) => Err(Io::Command(io)),
                }
            }
            #[cfg(feature = "keyring")]
            SubRead::Keyring(read) => {
                let output = match input {
                    None => read.resume(None),
                    Some(Io::Keyring(io)) => read.resume(Some(io)),
                    Some(io) => {
                        let err = format!("expected keyring input, got {io:?}");
                        return Err(Io::Error(err));
                    }
                };

                match output {
                    Ok(secret) => Ok(secret),
                    Err(io) => Err(Io::Keyring(io)),
                }
            }
        }
    }
}
