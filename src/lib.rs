//! Chargo library

pub mod error;

use rand::Rng;
use std::{
    fs::{read, write, File},
    io::{BufRead, BufReader, Read},
    path::PathBuf,
    vec,
};

use crate::error::Error;

use argon2::{self, Config};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305,
};

const HASH_SALT_LEN: usize = 16;
const NONCE: &[u8; 12] = &[0; 12];

/// Encrypts provided file with the specified password
/// `pwd` - password
/// `file_path` - file to encrypt
/// `output_path` - option file to write a result
/// ```
/// use chargo::encrypt_to_file;
/// use std::path::PathBuf;
///
/// fn main() {
///     encrypt_to_file("pwd".into(), PathBuf::from("test.txt"), Some(PathBuf::from("test.chargo"))).unwrap();
/// }
/// ```
pub fn encrypt_to_file(
    pwd: Vec<u8>,
    mut file_path: PathBuf,
    output_path: Option<PathBuf>,
) -> Result<bool, Error> {
    let file_data = read(&file_path)?;
    let data = encrypt(&pwd, &file_data)?;
    if let Some(opath) = output_path {
        file_path = opath
    }
    write(file_path, data)?;
    Ok(true)
}

/// Decrypts provided file with the specified password
/// `pwd` - password
/// `file_path` - file to decrypt
/// `output_path` - option file to write a result
/// ```
/// use chargo::decrypt_from_file;
/// use std::path::PathBuf;
///
/// fn main() {
///     decrypt_from_file("pwd".into(), PathBuf::from("test.chargo"), Some(PathBuf::from("test_dec.txt"))).unwrap();
/// }
/// ```
pub fn decrypt_from_file(
    pwd: Vec<u8>,
    mut file_path: PathBuf,
    output_path: Option<PathBuf>,
) -> Result<bool, Error> {
    let file = File::open(&file_path)?;
    let mut reader = BufReader::new(file);
    let data = decrypt(&pwd, &mut reader)?;
    if let Some(opath) = output_path {
        file_path = opath
    }
    write(file_path, data)?;
    Ok(true)
}

/// Encrypts provided bytes with the specified password
/// `pwd` - password
/// `data` - vec with data bytes
pub fn encrypt(pwd: &Vec<u8>, data: &Vec<u8>) -> Result<Vec<u8>, Error> {
    let config = Config::default();
    let rand_pwd = gen_bytes(32);
    let mut salt = gen_bytes(HASH_SALT_LEN);

    let (file_hash, mut file_cipher) = get_cipher(&rand_pwd, &salt, &config, data)?;
    let (_, mut key_cipher) = get_cipher(&pwd, &salt, &config, &file_hash)?;

    let mut final_data: Vec<u8> = vec![];
    final_data.append(&mut salt);
    final_data.push(b'\n');
    final_data.append(&mut key_cipher);
    final_data.push(b'\n');
    final_data.append(&mut file_cipher);

    Ok(final_data)
}

/// Decrypts data with the specified password
/// `pwd` - password
/// `reader` - buf reader
pub fn decrypt<BufReaderType>(
    pwd: &Vec<u8>,
    reader: &mut BufReader<BufReaderType>,
) -> Result<Vec<u8>, Error>
where
    BufReaderType: Read,
{
    let config = Config::default();

    let mut salt = vec![];
    reader.read_until(b'\n', &mut salt)?;
    salt.pop();

    let mut key_cipcher = vec![];
    reader.read_until(b'\n', &mut key_cipcher)?;
    key_cipcher.pop();

    let key_hash = get_hash(&pwd, &salt, &config)?;
    let file_hash = get_data(&key_hash, &key_cipcher)?;

    let mut ciphered_data = vec![];
    reader.read_to_end(&mut ciphered_data)?;

    Ok(get_data(&file_hash, &ciphered_data)?)
}

fn get_cipher(
    pwd: &Vec<u8>,
    salt: &Vec<u8>,
    config: &Config,
    data: &Vec<u8>,
) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let hash = get_hash(pwd, salt, config)?;
    let cipher = ChaCha20Poly1305::new((&hash[..]).into());
    let ciphered_data = cipher.encrypt(NONCE.into(), &data[..])?;
    Ok((hash, ciphered_data))
}

fn get_hash(pwd: &Vec<u8>, salt: &Vec<u8>, config: &Config) -> Result<Vec<u8>, Error> {
    Ok(argon2::hash_raw(&pwd[..], salt, config)?)
}

fn get_data(hash: &Vec<u8>, cipher_data: &Vec<u8>) -> Result<Vec<u8>, Error> {
    let cipher = ChaCha20Poly1305::new((&hash[..]).into());
    Ok(cipher.decrypt(NONCE.into(), &cipher_data[..])?)
}

fn gen_bytes(len: usize) -> Vec<u8> {
    const CHARSET: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();
    let password = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx]
        })
        .collect::<Vec<u8>>();
    password
}
