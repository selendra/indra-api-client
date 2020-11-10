extern crate crypto;
extern crate rand;

use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

use std::fs;
use std::path::Path;
use std::fs::File;
use std::str::{self, Utf8Error};
use std::io::prelude::*;
use std::fs::OpenOptions;

fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut encryptor = aes::cbc_encryptor(
            aes::KeySize::KeySize256,
            key,
            iv,
            blockmodes::PkcsPadding);
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = r#try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}




fn decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
            aes::KeySize::KeySize256,
            key,
            iv,
            blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = r#try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

/*
pub fn encrypt_mnemonic(mnemonic: String, password: String) -> String {
    let mut mnemonic_u8 = mnemonic.as_bytes();
    let mut password_u8 = password.as_bytes();

    let key = "ENCRYPTED_KEY";
    let env_value = dotenv::var(key).unwrap();
    let iv = env_value.as_bytes();
    let encrypted_data = encrypt(&mnemonic_u8, &password_u8, iv).ok().unwrap();

    //write to file
    let path = Path::new("user_encrypted.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(&encrypted_data) {
        Err(why) => return format!("couldn't write to {}: {}", display, why),
        Ok(_) => return format!("successfully wrote to {}", display),
    }
}
pub fn get_mnemonic(password: String) {
        let path = Path::new("user_encrypted.txt");
        let mut password_u8 = password.as_bytes();
        let key = "ENCRYPTED_KEY";
        let env_value = dotenv::var(key).unwrap();
        let iv = env_value.as_bytes();
        let read_result = fs::read(path).unwrap();
        let mut decrypt_result = decrypt(&read_result[..], &password_u8, &iv).ok().unwrap();
        println!("decrypt_result: {:?}", decrypt_result);
        let str_result = str::from_utf8(&decrypt_result).unwrap();
        println!("{}", str_result);
    
}
*/

use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
struct Mnemonics {
    mnemonics: Vec<Mnemonic_Encrypt>
}

// #[derive(Debug)]
#[derive(Debug, Deserialize, Serialize)]
struct Mnemonic_Encrypt {
    username: String,
    mnemonic: Vec<u8>,
    account_id: String,
}

pub fn encrypt_mnemonic(name: String, mnemonic: String, accountid: String, password: String) -> String {
    let mut mnemonic_u8 = mnemonic.as_bytes();
    let mut password_u8 = password.as_bytes();

    let key = "ENCRYPTED_KEY";
    let env_value = dotenv::var(key).unwrap();
    let iv = env_value.as_bytes();
    let encrypted_data = encrypt(&mnemonic_u8, &password_u8, iv).ok().unwrap();

    let path = Path::new("mnemonic_json.json");
    let display = path.display();

    if !path.exists() {
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
    }

    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open("mnemonic_json.json")
        .unwrap();

    println!("mnemonic_u8: {:?}", mnemonic_u8);
    println!("{:?}", String::from_utf8(mnemonic_u8.to_vec()));
    let mnemonic_encrypt = encrypt(&mnemonic_u8, &password.as_bytes(), iv).ok().unwrap();
    println!("mnemonic_encrypt1: {:?}", mnemonic_encrypt);

    let mut mnemonic_decrypted = decrypt(&mnemonic_encrypt[..], &password_u8, &iv).ok().unwrap();
    println!("mnemonic_decrypt: {:?}", String::from_utf8(mnemonic_decrypted.to_vec()));
    let user = Mnemonic_Encrypt {
        username: name,
        mnemonic:   mnemonic_encrypt,
        account_id: accountid,
    };

    let read_result = fs::read(path).expect("Error reading file in get_mnemonic");
    let str_result = str::from_utf8(&read_result).unwrap();
    
    if read_result.is_empty() {
        let mut vec_mnemonic: Vec<Mnemonic_Encrypt> = Vec::new();
        vec_mnemonic.push(user);
        let mnemonic_instance = Mnemonics {
            mnemonics: vec_mnemonic,
        };
        let mut json_mnemonic_str = serde_json::to_string_pretty(&mnemonic_instance).unwrap();
        match file.write_all(&json_mnemonic_str.as_bytes()) {
            Err(why) => {
                // file.close();
                // drop(file);
                return format!("couldn't write to {}: {}", display, why);
            },
            Ok(_) => {
                // file.close();
                // drop(file);
                return format!("successfully wrote to {}", display);
            }
        }
    } else {
        let mut data_deserialize: Mnemonics = serde_json::from_str(&str_result).unwrap();
        data_deserialize.mnemonics.push(user);

        let mnemonic_instance = Mnemonics {
            mnemonics: data_deserialize.mnemonics,
        };
        let mut json_mnemonic_str = serde_json::to_string_pretty(&mnemonic_instance).unwrap();
        match file.write_all(&json_mnemonic_str.as_bytes()) {
            Err(why) => {
                // file.close();
                // drop(file);
                return format!("couldn't write to {}: {}", display, why);
            },
            Ok(_) => {
                // file.close();
                // drop(file);
                return format!("successfully wrote to {}", display);
            }
        }
    }

}

pub fn get_mnemonic(name: String, password: String) {
    let mut password_u8 = password.as_bytes();
    let key = "ENCRYPTED_KEY";
    let env_value = dotenv::var(key).unwrap();
    let iv = env_value.as_bytes();

    let path = Path::new("mnemonic_json.json");
    let display = path.display();

    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap();

    let read_result = fs::read(path).expect("Error reading file in get_mnemonic");
    let str_result = str::from_utf8(&read_result).unwrap();
    let data_deserialize: Mnemonics = serde_json::from_str(&str_result).unwrap();
    let mnemonics = data_deserialize.mnemonics;
    for mut mnemonic_ in mnemonics {
        if mnemonic_.username.eq(&name) {
            println!("1: {:?}", mnemonic_.mnemonic);
            let mut mnemonic_decrypted = decrypt(&mnemonic_.mnemonic[..], &password_u8, &iv).ok().unwrap();
            println!("2: {:?}", mnemonic_decrypted);
            println!("result: {}", str::from_utf8(&mnemonic_decrypted).unwrap());
        } else {
            println!("hello");
        }
    }
}
