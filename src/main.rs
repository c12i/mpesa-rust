#![allow(unused)]
use dotenv;
use std::collections::HashMap;
use std::env;
use reqwest::blocking::Client;

use mpesa::{Mpesa, Environment};
use rsa::{PublicKey, RSAPrivateKey, RSAPublicKey, PaddingScheme};
use rand::rngs::OsRng;


fn main() {
    // test();
    test_rsa();
    // test_encryption();

    // let cer = Environment::Production.get_private_key();
    //
    // println!("{:?}",cer);
}

fn test() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let environment: Environment = "sandbox".parse()?;

    let client = Mpesa::new(
        env::var("CLIENT_KEY")?,
        env::var("CLIENT_SECRET")?,
        Environment::Sandbox, // or environment variable
    );

    let token = client.auth().unwrap();

    println!("token ==> {:?}", token);
    Ok(())
}

fn test_encryption() {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RSAPublicKey::from(&private_key);

    // Encrypt
    let data = "hello world".as_bytes();
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let enc_data = public_key.encrypt(&mut rng, padding, &data[..]).expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);

    // Decrypt
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let dec_data = private_key.decrypt(padding, &enc_data).expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);
}

fn test_rsa() {
    let file_content = r#"
-----BEGIN RSA PRIVATE KEY-----
MIIBOwIBAAJBAK5Z7jk1ql5DquRvlPmFgyBDCvdPQ0T2si2oPAUmNw2Z/qb2Sr/B
EBoWpagFf8Gl1K4PRipJSudDl6N/Vdb2CYkCAwEAAQJBAI3vWCfqsE8c9zoQPE8F
icHx0jOSq0ixLExO8M2gVqESq3SJpWbEbvPPbRb1sIqZHe5wV3Xmj09zvUzfdeB7
C6ECIQDjoB/kp7QlRiNhgudhQPct8XUf6Cgp7hBxL2K9Q9UzawIhAMQVvtH1TUOd
aSWiqrFx7w+54o58fIpkecI5Kl0TaWfbAiBrnye1Kn2IKhNMZWIUn2y+8izYeyGS
QZbQjQD4T3wcJQIgKGgWv2teNZ29ai0AIbrJuaLjhdsvStFzqctf6Hg0k1sCIQCj
JdwDGF7Kanex70KAacmOlw3vfx6XWT+2PH6Qh8tLug==
-----END RSA PRIVATE KEY-----
"#;

    let der_encoded = file_content
        .lines()
        .filter(|line| !line.starts_with("-"))
        .fold(String::new(), |mut data, line| {
            data.push_str(&line);
            data
        });
    let der_bytes = base64::decode(&der_encoded).expect("failed to decode base64 content");
    let private_key = RSAPrivateKey::from_pkcs1(&der_bytes).expect("failed to parse key");
    println!("private key ==> {:?}", private_key);
}