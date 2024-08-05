use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fmt::{Display, Formatter};
use thiserror::Error;
// Available if you need it!
// use serde_bencode

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Encoded {
    Int(i64),
    Float(f64),
    Text(String),
    List(Vec<Encoded>),
    Dict(HashMap<String, Encoded>),
}

#[derive(Debug,Error)]
enum Error {
    ParseError
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::ParseError => write!(f, "ParseError")
        }
    }
}

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> Result<Encoded, Error> {
    let type_char = encoded_value.chars().next().unwrap();
    if type_char.is_digit(10) {
        return decode_string(encoded_value);
    } else if type_char == 'i' {
        return decode_int(encoded_value);
    } else if type_char == 'l' {
        return decode_list(encoded_value);
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

fn decode_int(encoded: &str) -> Result<Encoded, Error> {
    let term_index = match encoded.find('e') {
        Some(idx) => idx,
        None => return Err(Error::ParseError)
    };

    let number = &encoded[1..term_index];
    let number = number.parse::<i64>().expect("Unable to parse int value");

    Ok(Encoded::Int(number))
}

fn decode_string(encoded: &str) -> Result<Encoded, Error> {
    let term_index = match encoded.find(':') {
        Some(idx) => idx,
        None => return Err(Error::ParseError)
    };

    let number = &encoded[..term_index];
    let number = match number.parse::<i64>() {
        Ok(n) => n,
        Err(_) => return Err(Error::ParseError)
    };

    let string = &encoded[term_index + 1..term_index + 1 + number as usize];

    Ok(Encoded::Text(string.to_string()))
}

fn decode_list(_: &str) -> Result<Encoded, Error> {
    Ok(Encoded::List(vec![]))
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: your_bittorrent.sh decode \"<encoded_value>\"");
        return;
    }

    let command = &args[1];

    if command == "decode" {
        let encoded = &args[2];

        let decoded = match decode_bencoded_value(encoded) {
            Ok(d) => d,
            Err(e) => panic!("Unable to decode value: {e}"),
        };

        let decoded_json = match serde_json::to_string(&decoded) {
            Ok(d) => d,
            Err(e) => panic!("Unable to deserialize: {e}"),
        };

        println!("{decoded_json}");
    } else {
        println!("unknown command: {}", args[1])
    }
}
