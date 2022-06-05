use std::fs::File;
use std::io::prelude::*;
use openssl::symm;
use tempfile::tempfile;

pub fn encrypt_file(mut file: File, cipher: symm::Cipher, key: [u8; 16], iv: &[u8; 16]){
    let mut temp = File::create(tempfile()).unwrap();

    loop {
        let mut chunk = Vec::with_capacity(chunk_size);
        let n = file.by_ref().take(chunk_size as u64).read_to_end(&mut chunk)?;
        if n == 0 { break; }
        temp.write_all(
            &*symm::encrypt(
                cipher,
                &key,
                Some(iv),
                &data).unwrap()
        ).expect("writing to file failed lol");
    }
}