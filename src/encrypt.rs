/// encryption is similar to hashing without destructive operation
/// it also requires a key - or something to encrypt against
/// For a Feistel cipher we'll have to generate subkeys from the master key
/// One subkey is used per round and there can be multiple rounds of encryption
/// One approach would be to hash a key to generate the subkey, which is hashed
/// to generate the next subkey...so on so forth
/// This will take a simpler approach so decryption subkey generation is easier
pub fn feistel_encrypt(message: &str, key: u8, rounds: u8) -> String {
    let mut left: Vec<u8> = message.bytes().collect();
    let message_length: usize = left.len();
    let mut right: Vec<u8> = left.split_off(message_length / 2);
    let mut subkey: u8;
    let mut updated_left: Vec<u8>;
    let mut updated_right: Vec<u8>;
    println!("about to encrypt\nleft {:?}\nright {:?}", left, right);
    for x in 0..rounds {
        subkey = key.rotate_right(x as u32);
        // L[i] = R[i - 1]
        updated_left = right.clone();
        // R[i] = L[i - 1] ⊕ f(r[i - 1], k[i])
        updated_right = Vec::new();
        right = encrypt_helper(right, subkey);
        for i in 0..left.len() {
            updated_right.push(left[i] ^ right[i]);
        }
        right = updated_right;
        left = updated_left;
    }
    println!("Finished all {:?} rounds\nleft {:?}\nright {:?}",
             rounds, left, right);
    let encrypted_left = String::from_utf8(left).unwrap();
    let encrypted_right = String::from_utf8(right).unwrap();
    let mut encrypted_message: String = String::new();
    encrypted_message.push_str(&encrypted_left);
    encrypted_message.push_str(&encrypted_right);
    println!("fully encrypted - {:?}", &encrypted_message);
    encrypted_message
}

fn encrypt_helper(right: Vec<u8>, subkey: u8) -> Vec<u8> {
    let mut added_right = Vec::new();
    for byte in right.clone() {
        byte.wrapping_add(subkey);
        added_right.push(byte);
    }
    added_right
}

