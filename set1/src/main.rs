use hex;
use base64;

// convert hex to base64
pub fn convert_hex_to_base64(input_hex_str: &str) -> String {
    let bytes = hex::decode(input_hex_str).unwrap();
    let base64_str = base64::encode(bytes);
    return base64_str;
}

// fixed XOR
pub fn fixed_xor(input_hex_str: &str, key_hex_str: &str) -> String {
    let input_hex_bytes = hex::decode(input_hex_str).unwrap();
    let key_hex_bytes = hex::decode(key_hex_str).unwrap();
    let output_hex_bytes = input_hex_bytes
        .iter()
        .zip(key_hex_bytes.iter())
        .map(|(&a, &b)| a ^ b)
        .collect::<Vec<u8>>();
    return hex::encode(output_hex_bytes);
}

fn main() {
    let input_hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64_str = convert_hex_to_base64(input_hex_str);
    println!("Challenge 1: {}", base64_str);

    let input_hex_str = "1c0111001f010100061a024b53535009181c";
    let key_hex_str = "686974207468652062756c6c277320657965";
    let output_hex_str = fixed_xor(input_hex_str, key_hex_str);
    println!("Challenge 2: {}", output_hex_str);
}