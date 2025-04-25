use std::io::{stdin, stdout, Write as _};

#[cfg(feature = "keyring")]
use io_keyring::{runtimes::std::handle as handle_keyring, Entry};
#[cfg(feature = "command")]
use io_process::{runtimes::std::handle as handle_process, Command};
use io_secret::{coroutines::Read, Io, Secret};
use secrecy::ExposeSecret;

fn main() {
    env_logger::init();

    let mut arg = None;
    let mut read = Read::new(secret());

    loop {
        match read.resume(arg) {
            Ok(secret) => break println!("secret: {}", secret.expose_secret()),
            Err(Io::Keyring(io)) => arg = Some(Io::Keyring(handle_keyring(io).unwrap())),
            Err(Io::Command(io)) => arg = Some(Io::Command(handle_process(io).unwrap())),
            Err(Io::Error(err)) => panic!("{err}"),
        }
    }
}

fn secret() -> Secret {
    match read_line("Backend (command, keyring)?").as_str() {
        #[cfg(feature = "command")]
        "command" => {
            let args = read_line("Command?");
            let mut args = args.split_whitespace();
            let mut cmd = Command::new(args.next().unwrap());
            cmd.args(args);
            Secret::Command(cmd)
        }
        #[cfg(not(feature = "command"))]
        "command" => {
            panic!("missing `command` cargo feature");
        }
        #[cfg(feature = "keyring")]
        "keyring" => {
            let name = read_line("Keyring entry name?");
            let entry = Entry::new(name);
            Secret::Keyring(entry)
        }
        #[cfg(not(feature = "keyring"))]
        "keyring" => {
            panic!("missing `keyring` cargo feature");
        }
        backend => {
            panic!("unknown backend {backend}");
        }
    }
}

fn read_line(prompt: &str) -> String {
    print!("{prompt} ");
    stdout().flush().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim().to_owned()
}
