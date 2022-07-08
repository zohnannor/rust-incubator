use rand::{prelude::SliceRandom, Rng};
use sha3::Digest;

fn main() {
    let chars: Vec<_> = "qwertyuiopasdfghjlkzxcvbnm123546789".chars().collect();
    let pass = generate_password(16, &chars).unwrap();
    let pass_hash = hash_password(&pass);
    dbg!(pass, pass_hash);

    dbg!(select_rand_val(&chars));

    let token = new_access_token();
    dbg!(token);

    let file_hash = get_file_hash(file!()).unwrap();
    eprintln!("{file_hash:x?}");
}

/// generates random password of given length and symbols set;
fn generate_password(len: usize, chars: &[char]) -> Option<String> {
    Some(
        rand::thread_rng()
            .sample_iter(&rand::distributions::Slice::new(chars).ok()?)
            .take(len)
            .collect(),
    )
}

/// retrieves random element from a given slice;
fn select_rand_val<T>(v: &[T]) -> Option<&T> {
    v.choose(&mut rand::thread_rng())
}

/// generates unique cryptographically secure random value in a-zA-Z0-9 symbols set and has exactly 64 symbols.
fn new_access_token() -> String {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}

/// returns SHA-3 hash of a file specified by its path.
fn get_file_hash(path: impl AsRef<std::path::Path>) -> std::io::Result<Vec<u8>> {
    Ok(sha3::Sha3_256::digest(std::fs::read_to_string(path)?).to_vec())
}

/// returns Argon2 password hash for a given password.
fn hash_password(password: &str) -> String {
    argonautica::Hasher::default()
        .with_password(password)
        .with_secret_key("dummy_secret_key")
        .hash()
        .unwrap()
}
