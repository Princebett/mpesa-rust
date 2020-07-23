#![allow(dead_code)]
use dotenv;
use std::collections::HashMap;
use std::env;
use reqwest::blocking::Client;

use openssl::x509::{X509};
use openssl::rsa::Padding;

use mpesa::{Mpesa, Environment};


fn main() {
    test();
}

fn test() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let environment: Environment = "sandbox".parse()?;

    let client = Mpesa::new(
        env::var("CLIENT_KEY")?,
        env::var("CLIENT_SECRET")?,
        Environment::Sandbox, // or environment variable
    );

    let token = client.get_security_credentials().unwrap();

    println!("token ==> {:?}", token);
    Ok(())
}


