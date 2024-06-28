use openssl::rsa::{Rsa};
use std::fs::{self, File};
use std::io::{Read, Write};
use crate::config::CricketConfig;
use std::error::Error;

pub fn initialize_keys(config: &CricketConfig) -> Result<(Rsa<openssl::pkey::Public>, Rsa<openssl::pkey::Private>), Box<dyn Error>> {
    let key_path = &config.key_path;

    if let Ok((public_key, private_key)) = load_keys(key_path) {
        Ok((public_key, private_key))
    } else {
        let rsa = Rsa::generate(2048)?;
        let public_key = rsa.public_key_to_pem()?;
        let private_key = rsa.private_key_to_pem()?;

        save_keys(key_path, &public_key, &private_key)?;

        Ok((Rsa::public_key_from_pem(&public_key)?, Rsa::private_key_from_pem(&private_key)?))
    }
}

fn load_keys(path: &str) -> Result<(Rsa<openssl::pkey::Public>, Rsa<openssl::pkey::Private>), Box<dyn Error>> {
    let mut pub_key_file = File::open(format!("{}.pub", path))?;
    let mut priv_key_file = File::open(format!("{}.pem", path))?;

    let mut pub_key = Vec::new();
    let mut priv_key = Vec::new();

    pub_key_file.read_to_end(&mut pub_key)?;
    priv_key_file.read_to_end(&mut priv_key)?;

    Ok((Rsa::public_key_from_pem(&pub_key)?, Rsa::private_key_from_pem(&priv_key)?))
}

fn save_keys(path: &str, public_key: &[u8], private_key: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut pub_key_file = File::create(format!("{}.pub", path))?;
    let mut priv_key_file = File::create(format!("{}.pem", path))?;

    pub_key_file.write_all(public_key)?;
    priv_key_file.write_all(private_key)?;

    Ok(())
}
