use openssl::symm;
use rand;

mod encrypt;
mod gather;

fn main() {
    let cipher = symm::Cipher::aes_128_cbc();
    let mut key: [u8; 16];
    let mut iv: [u8; 16];
    key.fill_with(rand::random::<u8>());
    iv.fill_with(rand::random::<u8>());
}
