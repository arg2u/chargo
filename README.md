# [![Latest Version](https://img.shields.io/crates/v/chargo.svg)](https://crates.io/crates/chargo) | [Documentation](https://docs.rs/chargo)

**Chargo** is a tool for file encryption/decryption with password. It's based on Argon2 and ChaCha20Poly1305 algorithms. From arg2u with ♥

## **Requirments**

To use Chargo you need to install Cargo and Rust.
Just paste into your terminal window:

```bash
curl https://sh.rustup.rs -sSf | sh
cargo install chargo
```

## **Usage**

```bash
chargo [OPTIONS] <mode> -i <input> -p <pwd>
```

## **Flags**

```bash
-h, --help       Prints help information
-V, --version    Prints version informat
```

## **Options**

```bash
-i <input>         Input file path
-o <output>        Out file path. If it is not provided, Chargo will override input file
-p <pwd>           Sets password
```

## **Args**

```bash
<mode>    Set mode decrypt or encrypt
```

## **Examples**

**In-Terminal usage**

```bash
chargo encrypt -p supadupapassword -i myfile.txt -o myfile.chargo
```
```bash
chargo decrypt -p supadupapassword -i myfile.chargo -o myfile.txt
```

**In-Code usage**
```rust
use chargo::{encrypt_to_file, decrypt_from_file};
use std::path::PathBuf;

fn main() {
    encrypt_to_file("pwd".into(), PathBuf::from("file.txt"), Some(PathBuf::from("file.chargo"))).unwrap();
    decrypt_from_file("pwd".into(), PathBuf::from("file.chargo"), Some(PathBuf::from("file.txt"))).unwrap();
}
```

## **Sponsor**

ETH: 0xd66e9d65EB278075859881A56B9027Da3260533E

## **License**

MIT