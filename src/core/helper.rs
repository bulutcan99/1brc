pub fn generate_hash_key(name: &[u8]) -> u64 {
    let mut key = [0u8; 8]; // An 8-byte array (initialized with 0).
    let l = name.len().min(8); // If the name length is greater than 8, only take the first 8 bytes.
    key[..l].copy_from_slice(&name[..l]); // Copies the first 8 bytes into the `key` array.
    key[0] ^= name.len() as u8; // XORs the length of the name into the first byte.
    u64::from_ne_bytes(key) // Converts the 8-byte array to a `u64`.
}

pub fn hash_key_to_str(key: &u64) -> String {
    format!("{}", key) // Converts the u64 to a String.
}
