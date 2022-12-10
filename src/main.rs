use spinners_rs::{Spinner, Spinners};
use std::path::PathBuf;
use structopt::StructOpt;

use chargo::{decrypt_from_file, encrypt_to_file, error::Error};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Chargo",
    about = "Chargo is a tool for file encryption/decryption with password. It's based on Argon2 and ChaCha20Poly1305 algorithms."
)]
struct Opt {
    /// Sets mode decrypt or encrypt
    mode: String,
    /// Sets password
    #[structopt(short = "p")]
    pwd: String,

    /// Input file path
    #[structopt(short = "i", parse(from_os_str))]
    input: PathBuf,

    /// Output file path. If it's not provided, Chargo will override input file.
    #[structopt(short = "o", parse(from_os_str))]
    output: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
enum Mode {
    /// Decryption mode
    Decrypt,
    /// Encryption mode
    Encrypt,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let mut sp = Spinner::new(Spinners::Aesthetic, "Encrypting ...");
    if opt.mode == "encrypt" {
        sp.start();
        encrypt_to_file(opt.pwd.into(), opt.input, opt.output)?;
    } else if opt.mode == "decrypt" {
        sp.set_message("Decrypting ...");
        sp.start();
        decrypt_from_file(opt.pwd.into(), opt.input, opt.output)?;
    } else {
        panic!("Wrong mode. It could be decrypt or encrypt");
    }
    Ok(())
}
